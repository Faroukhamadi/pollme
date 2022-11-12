import type { Handle } from '@sveltejs/kit';
import getParsedJwt from '$lib/utils/parseJwt';

export const handle: Handle = async ({ resolve, event }) => {
	const session = event.cookies.get('sid');
	if (!session) {
		return await resolve(event);
	}

	const user = getParsedJwt<typeof event.locals.user>(session);
	if (user) {
		event.locals.user = user;
	}

	return await resolve(event);
};
