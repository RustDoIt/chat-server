# Overview of the `server` library

The `server` library implements specialized Rust server nodes for media, text, and chat handling in a packet-based networked system. It integrates with `wg_internal` for networking and `common` for shared utilities, using channels for inter-thread communication and a routing handler for message routing.

Provides three server types:
- **MediaServer**: Stores and serves media files (e.g., images) via UUID-indexed HashMap.
- **TextServer**: Manages text files with content and media references.
- **ChatServer**: Handles client registration and message forwarding.

All servers implement the `Processor` trait for packet processing, command handling, and event notifications.

## Features

- Modular servers with shared `Processor` logic for commands, packets, and fragmentation.
- In-memory storage: HashMap for files (UUID keys), HashSet for chat clients.
- Request handling: Deserializes JSON requests (e.g., `WebRequest`, `ChatRequest`) and responds via routing.
- Command system: Supports node operations (add/remove neighbors, shutdown) and server-specific actions (add/remove files, get lists).
- Error management: Handles invalid UUIDs, missing files, malformed messages.
- File conversion: Uses `common::file_conversion` for path-to-file imports.
- Testing: Unit tests cover creation, operations, requests, and edge cases (e.g., large files, invalid inputs).

## Architecture

### Core Components

- **Processor Trait**: Interface for receivers (commands, packets), assembler, routing handler, message/command handling.
- **RoutingHandler**: Manages neighbors and message sending.
- **FragmentAssembler**: Reassembles packet fragments.
- **Types**: From `common::types`, includes commands/events (e.g., `NodeCommand`, `WebEvent`), requests/responses.
- **Common Commands (NodeCommand)**: Handled by all servers.
  - `AddSender(node_id, sender)`: Adds a neighbor to the routing handler.
  - `RemoveSender(node_id)`: Removes a neighbor from the routing handler.
  - `Shutdown`: Signals termination (returns true to exit processing loop).

### Server Details

#### MediaServer
- **Storage**: `HashMap<Uuid, MediaFile>` (title, chunked content).
- **Requests** (WebRequest variants):
  - `ServerTypeQuery`: Responds with `WebResponse::ServerType { server_type: ServerType::MediaServer }`; sends event `NodeEvent::ServerTypeQueried`.
  - `MediaQuery { media_id }`: Parses UUID, retrieves and serializes media file if found, responds with `WebResponse::MediaFile { media_data: ... }` and event `WebEvent::FileServed`; or `WebResponse::ErrorFileNotFound` if missing; or `WebResponse::BadUuid` if invalid (with event `WebEvent::BadUuid`).
- **Commands** (WebCommand variants):
  - `GetMediaFiles`: Retrieves all media files, sends event `WebEvent::MediaFiles { files: ... }`.
  - `GetMediaFile { media_id, location: _ }`: Retrieves specific media file if found, sends event `WebEvent::MediaFile { file: ... }`.
  - `AddMediaFile(media_file)`: Adds file to storage, sends event `WebEvent::MediaFileAdded { uuid: ... }`.
  - `AddMediaFileFromPath(file_path)`: Converts path to media file, adds if successful (sends `MediaFileAdded`), or sends `WebEvent::FileOperationError` on failure.
  - `RemoveMediaFile(uuid)`: Removes file if found (sends `WebEvent::MediaFileRemoved { uuid: ... }`), or sends `FileOperationError` if missing.

#### TextServer
- **Storage**: `HashMap<Uuid, TextFile>` (title, content, media refs).
- **Requests** (WebRequest variants):
  - `ServerTypeQuery`: Responds with `WebResponse::ServerType { server_type: ServerType::TextServer }`; sends event `NodeEvent::ServerTypeQueried`.
  - `TextFilesListQuery`: Retrieves formatted list ("UUID:title"), responds with `WebResponse::TextFilesList { files: ... }`; sends event `WebEvent::FilesListQueried`.
  - `FileQuery { file_id }`: Parses UUID, retrieves and serializes text file if found, responds with `WebResponse::TextFile { file_data: ... }` and event `WebEvent::FileServed`; or `WebResponse::ErrorFileNotFound` if missing; or `WebResponse::BadUuid` if invalid (with event `WebEvent::BadUuid`).
- **Commands** (WebCommand variants):
  - `GetTextFiles`: Retrieves all text files, sends event `WebEvent::TextFiles { files: ... }`.
  - `GetTextFile(uuid)`: Retrieves specific text file if found, sends event `WebEvent::TextFile { file: ... }`.
  - `AddTextFile(text_file)`: Adds file to storage, sends event `WebEvent::TextFileAdded { uuid: ... }`.
  - `AddTextFileFromPath(file_path)`: Converts path to text file, adds if successful (sends `TextFileAdded`), or sends `WebEvent::FileOperationError` on failure.
  - `RemoveTextFile(uuid)`: Removes file if found (sends `WebEvent::TextFileRemoved { uuid: ... }`), or sends `FileOperationError` if missing.

#### ChatServer
- **Storage**: `HashSet<NodeId>` for clients.
- **Requests** (ChatRequest variants):
  - `ServerTypeQuery`: Responds with `ChatResponse::ServerType { server_type: ServerType::ChatServer }`; sends event `NodeEvent::ServerTypeQueried`.
  - `RegistrationToChat { client_id }`: Registers client, responds with `ChatResponse::RegistrationSuccess`; sends event `ChatEvent::ClientRegistered`.
  - `ClientListQuery`: Retrieves registered client IDs, responds with `ChatResponse::ClientList { list_of_client_ids: ... }`; sends event `ChatEvent::ClientListQueried`.
  - `MessageFor { client_id, message }`: If target registered, forwards `ChatResponse::MessageFrom { client_id: from, message }` to target; else responds with `ChatResponse::ErrorWrongClientId`.
- **Commands** (ChatCommand variants):
  - `GetRegisteredClients`: Retrieves registered client IDs, sends event `ChatEvent::RegisteredClients { list: ... }`.