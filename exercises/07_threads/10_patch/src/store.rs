use crate::data::{Status, Ticket, TicketDraft, TicketPatch};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }

    pub fn update(&mut self, id: TicketId, ticket: TicketPatch) -> TicketId {
        let current_ticket = self.tickets.get_mut(&id).unwrap();
        if let Some(title) = ticket.title {
            current_ticket.title = title;
        }
        if let Some(description) = ticket.description {
            current_ticket.description = description;
        }
        if let Some(status) = ticket.status {
            current_ticket.status = status;
        }
        id
    }
}
