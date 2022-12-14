/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { invalid, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
	if (locals.user) {
		throw redirect(302, '/');
	}
};

export const actions: Actions = {
	default: async ({ cookies, request }) => {
		const data = await request.formData();
		const username = data.get('username');
		const password = data.get('password');
		try {
			const res = await fetch('http://localhost:3000/signup', {
				method: 'post',
				headers: {
					Accept: 'application/json',
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					username,
					password
				})
			});
			console.log('client_id: ', username);
			console.log('client_secret: ', password);
			console.log('response: ', res);
			if (!res.ok) {
				return invalid(400, { error: 'Invalid credentials', username });
			}
			const setCookieHeader = res.headers.get('set-cookie');
			const setCookieMap = new Map<string, string>();
			setCookieHeader?.split(';').forEach((cookie) => {
				const cookieAtributes = cookie.split('=');
				setCookieMap.set(
					cookieAtributes[0] && cookieAtributes[0].trim(),
					cookieAtributes[1] && cookieAtributes[1].trim()
				);
			});
			setCookieMap.forEach((value, key) => {
				console.log('key', key);
				console.log('value', value);
			});
			cookies.set('sid', setCookieMap.get('sid')!, {
				maxAge: parseInt(setCookieMap.get('Max-Age')!),
				path: setCookieMap.get('Path')!,
				httpOnly: setCookieMap.has('HttpOnly')!,
				secure: setCookieMap.has('Secure')!
			});
			throw redirect(302, '/');
		} catch (error) {
			console.log('we have an error', error);
			throw redirect(302, '/get-the-fuck-out-of-here');
		}
	}
};
