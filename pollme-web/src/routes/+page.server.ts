import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { DEV_ORIGIN } from '$lib/constants';
export interface Choice {
	id: number;
	name: string;
}

export interface Post {
	id: number;
	title: string;
	votes: string;
	vote: number;
	choice_count: number;
	created_at: string;
	choices: Choice[];
	hasVoted: boolean;
}

export const load: PageServerLoad = async ({ fetch, locals }) => {
	if (!locals.user) {
		throw redirect(307, '/login');
	}
	const res = await fetch(`${DEV_ORIGIN}/posts`, {
		headers: {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		}
	});

	const posts: Post[] = await res.json();

	for (let i = 0; i < posts.length; i++) {
		const res = await fetch(`${DEV_ORIGIN}/posts/${posts[i].id}/choices`);
		const choices: Choice[] = await res.json();
		posts[i].choices = choices;
		for (let j = 0; j < posts[i].choices.length; j++) {
			const res = await fetch(`${DEV_ORIGIN}/choices/${posts[i].choices[i].id}/${locals.user.sub}`);
			console.log('resssss: ', res);
			const thingie = await res.json();
			console.log('res inside inside: ' + thingie);
			// "/choices/:id/:user_id"
		}
	}

	return {
		posts
	};
};
