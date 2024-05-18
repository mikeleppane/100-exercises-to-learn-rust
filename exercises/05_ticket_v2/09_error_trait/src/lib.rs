use std::fmt::Debug;

#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

impl std::error::Error for TicketNewError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TicketNewError::TitleError(_) => None,
            TicketNewError::DescriptionError(_) => None,
        }
    }
}

impl std::fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TicketNewError::TitleError(msg) => msg,
                TicketNewError::DescriptionError(msg) => msg,
            }
        )
    }
}

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    let description = if description.is_empty() || description.len() > 500 {
        "Description not provided".to_string()
    } else {
        description
    };

    Ticket::new(title, description, status).unwrap()
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError(
                "Title cannot be empty".to_string(),
            ));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError(
                "Title cannot be longer than 50 characters".to_string(),
            ));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be empty".to_string(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be longer than 500 characters".to_string(),
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};
    use static_assertions::assert_impl_one;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    fn display_is_correctly_implemented() {
        let ticket = Ticket::new("".into(), valid_description(), Status::ToDo);
        assert_eq!(format!("{}", ticket.unwrap_err()), "Title cannot be empty");
    }

    assert_impl_one!(TicketNewError: std::error::Error);
}
