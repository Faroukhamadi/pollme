import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { DEV_ORIGIN } from '$lib/constants';
export interface Choice {
	id: number;
	name: string;
	post_id: number;
	count: number;
}

export interface Post {
	id: number;
	title: string;
	votes: number;
	created_at: string;
	choices: Choice[];
	choicesCount: number;
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
		posts[i].hasVoted = false;

		const res = await fetch(`${DEV_ORIGIN}/posts/${posts[i].id}/choices`);

		const choices: Choice[] = await res.json();
		posts[i].choices = choices;

		for (let j = 0; j < posts[i].choices.length; j++) {
			const res = await fetch(
				`${DEV_ORIGIN}/choices/${posts[i].choices[j].name}/${locals.user.sub}`
			);

			const choice: Choice = await res.json();

			if (posts[i].hasVoted === false) {
				posts[i].hasVoted = choice ? true : false;
			}

			if (posts[i].hasVoted) {
				const res = await fetch(`${DEV_ORIGIN}/choices/count/${posts[i].choices[j].post_id}`);
				const totalCount: number = await res.json();
				posts[i].choicesCount = totalCount;

				for (let k = 0; k < posts[i].choices.length; k++) {
					const res2 = await fetch(`${DEV_ORIGIN}/choices/${posts[i].choices[k].name}/count`);
					const choiceCount: number = parseInt(await res2.text());
					posts[i].choices[k].count = choiceCount;
					console.log('choice count: ', choiceCount);
				}
			}
		}
	}

	return {
		posts,
		time: new Date()
	};
};
