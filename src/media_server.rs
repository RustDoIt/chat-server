use std::any::Any;
use std::collections::{HashMap};
use crossbeam::channel::{Receiver, Sender};
use uuid::{Uuid};
use wg_internal::network::NodeId;
use wg_internal::packet::{NodeType, Packet};
use common::{FragmentAssembler, RoutingHandler};
use common::packet_processor::Processor;
use common::types::{File, MediaFile, NodeCommand, ServerType, WebCommand, WebEvent, WebRequest, WebResponse};

pub struct MediaServer {
    routing_handler: RoutingHandler,
    controller_recv: Receiver<Box<dyn Any>>,
    controller_send: Sender<Box<dyn Any>>,
    packet_recv: Receiver<Packet>,
    id: NodeId,
    assembler: FragmentAssembler,
    stored_media: HashMap<Uuid, MediaFile>,
}

impl MediaServer {
    pub fn new(
        id: NodeId,
        neighbors: HashMap<NodeId, Sender<Packet>>,
        packet_recv: Receiver<Packet>,
        controller_recv: Receiver<Box<dyn Any>>,
        controller_send: Sender<Box<dyn Any>>
    ) -> Self {
        let router = RoutingHandler::new(id, NodeType::Server, neighbors, controller_send.clone());
        Self {
            routing_handler: router,
            controller_recv,
            controller_send,
            packet_recv,
            id,
            assembler: FragmentAssembler::default(),
            stored_media: HashMap::new(),
        }
    }

    fn get_media_by_id(&self, media_id: Uuid) -> Option<&MediaFile> {
        todo!()
    }

    pub fn add_media_file(&mut self, media_file: MediaFile) {
        todo!()
    }

    pub fn remove_media_file(&mut self, media_id: Uuid) -> Option<MediaFile> {
        todo!()
    }

    fn get_all_media_files(&self) -> Vec<MediaFile> {
        todo!()
    }

    fn get_media_list(&self) -> Vec<String> {
        todo!()
    }
}

impl Processor for MediaServer {
    fn controller_recv(&self) -> &Receiver<Box<dyn Any>> {
        &self.controller_recv
    }

    fn packet_recv(&self) -> &Receiver<Packet> {
        &self.packet_recv
    }

    fn assembler(&mut self) -> &mut FragmentAssembler {
        &mut self.assembler
    }

    fn routing_handler(&mut self) -> &mut RoutingHandler {
        &mut self.routing_handler
    }

    fn handle_msg(&mut self, msg: Vec<u8>, from: NodeId, session_id: u64) {
        if let Ok(msg) = serde_json::from_slice::<WebRequest>(&msg) {
            match msg {
                WebRequest::ServerTypeQuery => {
                    if let Ok(res) = serde_json::to_vec(&WebResponse::ServerType { server_type: ServerType::MediaServer }) {
                        let _ = self.routing_handler.send_message(&res, from, Some(session_id));
                    }
                }

                WebRequest::TextFilesListQuery => {}
                WebRequest::FileQuery { .. } => {}
                WebRequest::MediaQuery { .. } => {}
            }
        }
    }

    fn handle_command(&mut self, cmd: Box<dyn Any>) -> bool {
        if let Some(cmd) = cmd.downcast_ref::<NodeCommand>() {
            match cmd {
                NodeCommand::AddSender(node_id, sender) => self.routing_handler.add_neighbor(*node_id, sender.clone()),
                NodeCommand::RemoveSender(node_id) => self.routing_handler.remove_neighbor(*node_id),
                NodeCommand::Shutdown => return true
            }
        }  else if let Some(cmd) = cmd.downcast_ref::<WebCommand>() {
            match cmd {
                WebCommand::GetCachedFiles => {}
                WebCommand::GetFile(_) => {}
                WebCommand::GetTextFiles => {}
                WebCommand::GetTextFile(_) => {}
                WebCommand::GetMediaFiles => {}
                WebCommand::GetMediaFile(_) => {}
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam::channel::unbounded;

    #[test]
    fn test_() {

    }
}