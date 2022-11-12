export default function getParsedJwt<T extends object = { [k: string]: string | number }>(
	token: string
): T | undefined {
	try {
		return JSON.parse(Buffer.from(token.split('.')[1], 'base64').toString());
	} catch {
		return undefined;
	}
}
