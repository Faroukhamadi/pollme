// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
	interface Locals {
		token?: string;
		user: {
			sub: string;
			username: string;
			exp: string;
		};
	}
	// interface PageData {}
	// interface Error {}
	// interface Platform {}
}
