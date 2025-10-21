// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum TicketType {
    Standard,
    Backstage(String),
    Vip(String)
}

#[derive(Debug)]
struct Ticket {
    ticket_tier : TicketType,
    price : u64,
}

impl Ticket {
    fn new(tier : TicketType, price: u64) -> Self {
        Self {
            ticket_tier : tier,
            price
        }
    }
}

fn main() {
    let ticket_1 = Ticket::new(TicketType::Standard, 64);
    let ticket_2 = Ticket::new(TicketType::Backstage("Wande".to_owned()), 100);
    let ticket_3 = Ticket::new(TicketType::Vip("Enoch".to_owned()), 150);

    let mut tickets : Vec<Ticket> = Vec::new();
    tickets.push(ticket_1);
    tickets.push(ticket_2);
    tickets.push(ticket_3);

    for ticket in tickets {
        match ticket.ticket_tier {
            TicketType::Standard => println!("Standard  TicketPrice : {:?}", ticket.price),
            TicketType::Backstage(holder) => println!("Backstage ticket for : {:?}, Price : {:?}", holder, ticket.price),
            TicketType::Vip(holder) => println!("Vip ticket for : {:?}Price : {:?}", holder, ticket.price)
        }
    }
}

