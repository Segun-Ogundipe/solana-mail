use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct Mail {
  pub id: String,
  pub from_address: String,
  pub to_address: String,
  pub subject: String,
  pub body: String,
  pub sent_date: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MailAccount {
  pub inbox: Vec<Mail>,
  pub sent: Vec<Mail>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct DataLength {
  pub length: u32,
}

// Sanity tests
#[cfg(test)]
mod test {
  use super::*;
  use solana_program::{borsh::get_instance_packed_len, pubkey::Pubkey};

  #[test]
  fn test_mail() {
    let mail = Mail {
      id: String::from("00000000-0000-0000-0000-000000000000"),
      from_address: Pubkey::default().to_string(),
      to_address: Pubkey::default().to_string(),
      subject: String::from("Hey Mike"),
      body: String::from("Body text with some characters"),
      sent_date: String::from("9/29/2021, 3:58:02 PM"),
    };

    let mut temp_slice = [0; 500];

    mail.serialize(&mut &mut temp_slice[..]).unwrap();

    let mail =
      Mail::try_from_slice(&temp_slice[..get_instance_packed_len(&mail).unwrap()]).unwrap();

    assert_eq!(mail.subject, "Hey Mike");
  }
  #[test]
  fn test_mail_account() {
    let mail = Mail {
      id: String::from("00000000-0000-0000-0000-000000000000"),
      from_address: Pubkey::default().to_string(),
      to_address: Pubkey::default().to_string(),
      subject: String::from("Hey Mike"),
      body: String::from("Body text with some characters"),
      sent_date: String::from("9/29/2021, 3:58:02 PM"),
    };

    let mail_account = MailAccount {
      inbox: vec![mail],
      sent: Vec::new(),
    };

    let mut temp_slice = [0; 500];

    mail_account.serialize(&mut &mut temp_slice[..]).unwrap();

    let mail_account =
      MailAccount::try_from_slice(&temp_slice[..get_instance_packed_len(&mail_account).unwrap()])
        .unwrap();

    assert_eq!(mail_account.inbox[0].subject, "Hey Mike");
  }

  #[test]
  fn test_data_length() {
    let data_length = DataLength { length: 5 };

    let mut temp_slice = [0; 4];

    data_length.serialize(&mut &mut temp_slice[..]).unwrap();

    assert_eq!(temp_slice, [5, 0, 0, 0]);

    let data_length = DataLength::try_from_slice(&temp_slice[..4]).unwrap();

    assert_eq!(data_length.length, 5);
  }
}
