import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get('sid');
	if (token) {
		// event.locals.token = token;
		event.locals.token = token;
	}
	console.log('token: ', token);

	return await resolve(event);
};
