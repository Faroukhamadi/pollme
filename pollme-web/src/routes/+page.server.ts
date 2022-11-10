import type { PageServerLoad } from './$types';

export interface Post {
	title: string;
	first_choice: string;
	second_choice: string;
	votes: string;
	first_choice_count: number;
	second_choice_count: number;
	choice_count: number;
	created_at: string;
}

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch('http://localhost:3000/posts', {
		headers: {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		}
	});
	console.log('status code: ', res.status);
	const posts: Post[] = await res.json();
	console.log('posts: ', posts);

	return {
		posts
	};
};
