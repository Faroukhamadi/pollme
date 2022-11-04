import type { PageLoad } from './$types';

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

export const load: PageLoad = async ({ fetch }) => {
	const posts: Post[] = await (await fetch('http://localhost:3000')).json();

	return {
		posts
	};
};
