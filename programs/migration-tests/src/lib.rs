use anchor_lang::prelude::*;

declare_id!("AkKZAdS6NjcJGAtuWDaZAACXgNBnAgrw4U9hcNewe9Xh");

#[program]
pub mod migration_tests {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.data.bump = *ctx.bumps.get("data").unwrap();
        ctx.accounts.data.data = vec![0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1];
        Ok(())
    }

    pub fn migrate_big_to_small(_ctx: Context<MigrateBigToSmall>) -> Result<()> {
        Ok(())
    }

    pub fn migrate_small_to_big(_ctx: Context<MigrateSmallToBig>) -> Result<()> {
        Ok(())
    }

    pub fn migrate_big_to_bigger(_ctx: Context<MigrateBigToBigger>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = TypeB::INIT_SPACE,
        seeds = [b"account", signer.key().as_ref()],
        bump
    )]
    data: Account<'info, TypeB>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MigrateBigToSmall<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"account", signer.key().as_ref()],
        bump = data.bump,
        realloc = TypeA::INIT_SPACE,
        realloc::zero = false, 
        realloc::payer = signer
    )]
    data: Migration<'info, TypeB, TypeA>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MigrateSmallToBig<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"account", signer.key().as_ref()],
        bump = data.bump,
        realloc = TypeB::INIT_SPACE,
        realloc::zero = false, 
        realloc::payer = signer
    )]
    data: Migration<'info, TypeA, TypeB>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MigrateBigToBigger<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"account", signer.key().as_ref()],
        bump = data.bump,
        realloc = TypeC::INIT_SPACE,
        realloc::zero = false, 
        realloc::payer = signer
    )]
    data: Migration<'info, TypeB, TypeC>,
    system_program: Program<'info, System>
}

#[account]
pub struct TypeA {
    pub bump: u8,
    pub data: Vec<u8>
}

impl Space for TypeA {
    const INIT_SPACE: usize = 8 + 1 + 4 + 32;
}

impl Migrate<TypeB> for TypeA {
    fn migrate(&self) -> TypeB {
        TypeB {
            bump: self.bump,
            data: vec![0x0b; 64]
        }
    }
}

#[account]
pub struct TypeB {
    pub bump: u8,
    pub data: Vec<u8>
}

impl Migrate<TypeA> for TypeB {
    fn migrate(&self) -> TypeA {
        TypeA {
            bump: self.bump,
            data: vec![0x0a;32]
        }
    }
}

impl Space for TypeB  {
    const INIT_SPACE: usize = 8 + 1 + 4 + 64;
}

#[account]
pub struct TypeC {
    pub bump: u8,
    pub data: Vec<u8>
}

impl Migrate<TypeC> for TypeB {
    fn migrate(&self) -> TypeC {
        TypeC {
            bump: self.bump,
            data: vec![0x0c;256]
        }
    }
}

impl Space for TypeC  {
    const INIT_SPACE: usize = 8 + 1 + 4 + 256;
}