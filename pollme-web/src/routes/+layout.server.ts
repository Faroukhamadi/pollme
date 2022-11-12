import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
	return {
		user: locals.user
	};
};
// export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
// 	const thingie = await fetch('http://localhost:3000/authorize', {
// 		method: 'post',
// 		headers: {
// 			Accept: 'application/json',
// 			'Content-Type': 'application/json'
// 		},
// 		body: JSON.stringify({
// 			client_id: 'farouk',
// 			client_secret: 'password123'
// 		})
// 	});

// 	console.log('thingie: ', thingie);

// 	// let session = cookies.get('sessionid');
// 	// if (!session || session.length === 0) {
// 	// 	session = '123';
// 	// }
// 	// console.log('session: ', session);
// 	// return {
// 	// 	user: session
// 	// };
// };
