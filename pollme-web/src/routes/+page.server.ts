import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export interface Post {
	id: number;
	title: string;
	first_choice: string;
	second_choice: string;
	votes: string;
	vote: number;
	first_choice_count: number;
	second_choice_count: number;
	choice_count: number;
	created_at: string;
}

export const load: PageServerLoad = async ({ fetch, locals }) => {
	if (!locals.user) {
		throw redirect(307, '/login');
	}
	const res = await fetch('http://localhost:3000/posts', {
		headers: {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		}
	});

	console.log('response is: ', res);

	const posts: Post[] = await res.json();
	console.log('posts: ', posts);

	return {
		posts
	};
};
