
//Alias ensure we don't confuse these types with other strings or numbers
type Address = String;
type Amount = i64;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    pub sender: Address,
    pub recipient: Address,
    pub amount: Amount
}

impl Transaction {
    pub fn new(sender: Address, recipient: Address, amount: Amount) -> Transaction {
        Transaction {
            sender: sender,
            recipient: recipient,
            amount: amount
        }
    }
}
