// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod myepicproject {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
    use super::*;
    // Lastly, we do this thing in our function where we just grab base_account from the StartStuffOff context by doing Context<StartStuffOff>.
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // get a refrence to the account
        let base_account = &mut ctx.accounts.base_account;
        //initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    // The function now accepts a gif_link param from the user. We also reference the user from the Context

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        // get a reference to the account and increment total_gifs
        //  grab the base_account which was passed in to the function via Context<AddGif>. Then, I increment the counter
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        // build the struct
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };
        // add it to the gif_list vector
        base_account.gif_list.push(item);

        base_account.total_gifs += 1;
        Ok(())
    }
}

// Specify what data you want in the AddGif Context.
// I create a Context named AddGif that has access to a mutable reference to base_account. That's why I do #[account(mut)]. Basically it means I can actually change the total_gifs value stored on BaseAccount.
// Otherwise, I may change data on it within my function but it wouldn't actually change on my account. Now, w/ a "mutable" reference if I mess w/ base_account in my function it'll change data on the account itself.
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    // Add the signer who calls the AddGif method to the struct so that we can save it
    #[account(mut)]
    pub user: Signer<'info>,
}
// Hope you can kinda see how the Context we set up near the bottom of the program actually becomes useful within the function. It's basically a nice way to say, "Hey, when someone calls add_gif be sure to attach the AddGif context to it as well so the user can access the base_account and whatever else is attached to AddGif.

// create a custon struct for us to work with
// It's a little complex, but, basically this tells Anchor how to serialize/deserialize the struct. Remember, data is being stored in an "account" right? That account is basically a file and we serialize our data into binary format before storing it. Then, when we want to retrieve it we'll actually deserialize it.
// This line takes care of that to make sure our data is properly serialized/deserialized since we're creating a custom struct here.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// attache variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    //  All we're doing here is telling Solana how we want to initialize BaseAccount.
    // init will tell Solana to create a new account owned by our current program.
    // payer = user tells our program who's paying for the account to be created. In this case, it's the user calling the function.
    // We then say space = 9000 which will allocate 9000 bytes of space for our account. You can change this # if you wanted, but, 9000 bytes is enough for the program we'll be building here!
    #[account(init, payer = user, space =9000)]
    pub base_account: Account<'info, BaseAccount>,
    // Note: We do &mut to get a "mutable reference" to base_account. When we do this it actually gives us the power to make changes to base_account. Otherwise, we'd simply be working w/ a "local copy" of base_account.
    #[account(mut)]
    // pub user: Signer<'info> which is data passed into the program that proves to the program that the user calling this program actually owns their wallet account.
    pub user: Signer<'info>,
    // pub system_program: Program which is actually pretty freaking cool. It's basically a reference to the SystemProgram. The SystemProgram is the program that basically runs Solana. It is responsible for a lot of stuff, but one of the main things it does is create accounts on Solana. The SystemProgram is a program the creators of Solana deployed that other programs like ours talk to
    pub system_program: Program<'info, System>,
}

// tell solana we want to store on this account
// Basically, it tells our program what kind of account it can make and what to hold inside of it. So, here, BaseAccount holds one thing and it's an integer named total_gifs.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account. (array type)
    pub gif_list: Vec<ItemStruct>,
}