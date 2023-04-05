use rusqlite::{params, Connection, Result as SqliteResult};

trait Account {
    fn deposit(&mut self, amount: f32);
    fn withdraw(&mut self, amount: f32) -> Result<(), &'static str>;
    fn balance(&self) -> f32;
}

struct BankAccount {
    balance: f32,
    id: i32
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f32) -> Result<(), &'static str> {
        if amount > self.balance {
            return Err("Insufficient funds");
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f32 {
        self.balance
    }
}

use rusqlite::{params, Connection, Result as SqliteResult};

struct SqliteAccountRepository {
    conn: Connection
}

impl SqliteAccountRepository {
    fn new() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance REAL)",
            params![],
        )?;
        Ok(SqliteAccountRepository{conn})
    }
}

impl AccountRepository for SqliteAccountRepository {
    fn find_account(&self, id: &i32) -> Result<BankAccount, &'static str> {
        let mut stmt = self.conn.prepare("SELECT id, balance FROM accounts WHERE id = ?")?;
        let account_iter = stmt.query_map(&[id], |row| {
            Ok(BankAccount {
                id: row.get(0)?,
                balance: row.get(1)?,
            })
        })?;
        let account = account_iter.next().ok_or("Account not found")??;
        Ok(account)
    }

    fn save_account(&self, account: BankAccount) -> Result<(), &'static str> {
        self.conn.execute("INSERT INTO accounts (id, balance) VALUES (?, ?)",
                 params![account.id, account.balance])?;
        Ok(())
    }
}

#[derive(Debug)]
enum AccountError {
    NotFound,
    InsufficientFunds,
    RepositoryError(String),
}

impl From<&'static str> for AccountError {
    fn from(err: &'static str) -> Self {
        match err {
            "Account not found" => AccountError::NotFound,
            "Insufficient funds" => AccountError::InsufficientFunds,
            _ => AccountError::RepositoryError(err.to_string()),
        }
    }
}

impl From<String> for AccountError {
    fn from(err: String) -> Self {
        AccountError::RepositoryError(err)
    }
}

impl From<rusqlite::Error> for AccountError {
    fn from(err: rusqlite::Error) -> Self {
        AccountError::RepositoryError(err.to_string())
    }
}

struct AccountService<'a> {
    repository: &'a dyn AccountRepository
}

impl AccountService<'_> {
    fn new(repository: &dyn AccountRepository) -> Self {
        Self { repository }
    }

    fn deposit(&self, id: i32, amount: f32) -> Result<(), AccountError> {
        let mut account = self.repository.find_account(&id)?;
        account.deposit(amount);
        self.repository.save_account(account)?;
        Ok(())
    }

    fn withdraw(&self, id: i32, amount: f32) -> Result<(), AccountError> {
        let mut account = self.repository.find_account(&id)?;
        account.withdraw(amount).map_err(|e| e.into())?;
        self.repository.save_account(account)?;
        Ok(())
    }
}
