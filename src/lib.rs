use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// 定义存储在账户中的状态类型
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// 问候次数
    pub counter: u32,
}

// 声明并导出程序的入口点
entrypoint!(process_instruction);

/// 程序入口点的实现
pub fn process_instruction(
    program_id: &Pubkey, // 加载 Hello World 程序的账户的公钥
    accounts: &[AccountInfo], // 要问候的账户
    _instruction_data: &[u8], // 被忽略,所有 helloworld 指令都是问候
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // 迭代账户比直接索引更安全
    let accounts_iter = &mut accounts.iter();

    // 获取要问候的账户
    let account = next_account_info(accounts_iter)?;

    // 账户必须由程序拥有才能修改其数据
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // 增加并存储账户被问候的次数
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut *account.data.borrow_mut())?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}
