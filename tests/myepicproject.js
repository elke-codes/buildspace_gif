// const anchor = require("@project-serum/anchor");

// describe("myepicproject", () => {
// 	// Configure the client to use the local cluster.
// 	anchor.setProvider(anchor.Provider.env());

// 	it("Is initialized!", async () => {
// 		// Add your test here.
// 		const program = anchor.workspace.Myepicproject;
// 		const tx = await program.rpc.initialize();
// 		console.log("Your transaction signature", tx);
// 	});
// });

const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

const main = async () => {
	console.log("Starting test");

	//   // Create and set the provider. We set it before but we needed to update it, so that it can communicate with our frontend!

	// anchor.setProvider(anchor.Provider.env());

	const provider = anchor.Provider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace.Myepicproject;

	//   // Create an account keypair for our program to use.
	// anchor.web3.Keypair.generate() may also be kinda confusing — why are we doing this? Well, basically it's because we need to create some credentials for the BaseAccount we're creating.
	const baseAccount = anchor.web3.Keypair.generate();

	//   // Call start_stuff_off, pass it the params it needs!
	// Note: notice also that in lib.rs the function is called start_stuff_off since in Rust we use snake case (snake_case) instead of camel case. But, over in our javascript file we use camel case and actually call startStuffOff. This is something nice Anchor does for us so we can follow best practices regardless of what language we're using. You can use snake case in Rust-land and camel case in JS-land.
	let tx = await program.rpc.startStuffOff({
		accounts: {
			baseAccount: baseAccount.publicKey,
			user: provider.wallet.publicKey,
			systemProgram: SystemProgram.programId
		},
		signers: [baseAccount]
	});

	console.log("your transaction signature", tx);

	// Fetch data from the account.
	let account = await program.account.baseAccount.fetch(
		baseAccount.publicKey
	);
	console.log("gif count", account.totalGifs.toString());

	// call add_gif
	//   // You'll need to now pass a GIF link to the function! You'll also need to pass in the user submitting the GIF!

	await program.rpc.addGif(
		"https://giphy.com/gifs/roverdotcom-rover-border-collie-gifmedogs-jGRHaDpv4Y4mRU5hkF",
		{
			accounts: {
				baseAccount: baseAccount.publicKey,
				user: provider.wallet.publicKey
			}
		}
	);

	// get the account again to see what changed
	account = await program.account.baseAccount.fetch(baseAccount.publicKey);
	console.log("gif count", account.totalGifs.toString());

	// access gif_list on the account
	console.log("gif list", account.gifList);
};

const runMain = async () => {
	try {
		await main();
		process.exit(0);
	} catch (error) {
		console.error(error);
		process.exit(1);
	}
};

runMain();
