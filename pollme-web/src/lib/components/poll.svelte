<script lang="ts">
	import delay from '$lib/utils/delay';
	import timeSince from '$lib/utils/timeSince';
	import percentage from '$lib/utils/percentage';
	import { page } from '$app/stores';
	import { DEV_ORIGIN } from '$lib/constants';
	import type { Post } from '../../routes/+page.server';

	export let post: Post;

	let isFetching = false;
	enum Vote {
		Downvote = -1,
		Upvote = 1
	}
</script>

<div class="flex gap-5 bg-indigo-200 rounded-md m-4 p-4">
	<div class="flex flex-col justify-center">
		<button
			disabled={isFetching}
			class="disabled:cursor-pointer"
			on:click={() => {
				isFetching = true;
				fetch(`${DEV_ORIGIN}/posts/${post.id}/vote?id=${Vote.Upvote}`, {
					credentials: 'include',
					method: 'POST'
				})
					.then(() => {
						delay(1500)
							.then(() => {
								isFetching = false;
								if (post.vote === 1) {
									post.votes -= 1;
									post.vote = 0;
								} else if (post.vote === -1) {
									post.votes = post.votes + 2;
									post.vote = 1;
								} else if (post.vote === 0) {
									post.votes += 1;
									post.vote = 1;
								}
							})
							.catch((e) => console.error(e));
					})
					.catch((e) => console.error(e));
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				aria-hidden="true"
				class="h-7 w-7 text-neutral-500 hover:rounded-sm hover:bg-neutral-700 hover:bg-neutral-600 hover:text-green-400"
				class:text-green-400={post.vote === 1 ? true : false}
				><path
					fill-rule="evenodd"
					d="M11.47 7.72a.75.75 0 011.06 0l7.5 7.5a.75.75 0 11-1.06 1.06L12 9.31l-6.97 6.97a.75.75 0 01-1.06-1.06l7.5-7.5z"
					clip-rule="evenodd"
				/></svg
			>
		</button>
		<p class="text-center">{post.votes}</p>
		<button
			disabled={isFetching}
			class="disabled:cursor-pointer"
			on:click={() => {
				isFetching = true;
				fetch(`${DEV_ORIGIN}/posts/${post.id}/vote?id=${Vote.Downvote}`, {
					credentials: 'include',
					method: 'POST'
				})
					.then(() => {
						delay(1500)
							.then(() => {
								isFetching = false;
								if (post.vote === 1) {
									post.votes -= 2;
									post.vote = -1;
								} else if (post.vote === -1) {
									post.votes += 1;
									post.vote = 0;
								} else if (post.vote === 0) {
									post.votes -= 1;
									post.vote = -1;
								}
							})
							.catch((e) => console.error(e));
					})
					.catch((e) => console.error(e));
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				aria-hidden="true"
				class:text-red-400={post.vote === -1 ? true : false}
				class="h-7 w-7 text-neutral-500 hover:rounded-sm hover:bg-neutral-700 hover:bg-neutral-600 hover:text-red-400"
				><path
					fill-rule="evenodd"
					d="M12.53 16.28a.75.75 0 01-1.06 0l-7.5-7.5a.75.75 0 011.06-1.06L12 14.69l6.97-6.97a.75.75 0 111.06 1.06l-7.5 7.5z"
					clip-rule="evenodd"
				/></svg
			>
		</button>
	</div>
	<div class="flex flex-col">
		<h3 class="text-xl">{post.title.slice(50)}...</h3>
		<div class="flex gap-2">
			<!-- starts here -->
			{#if post.hasVoted}
				{#each post.choices as choice}
					<div class="btn btn-disabled bg-opacity-30  my-2 flex gap-5">
						<div
							class="radial-progress"
							style={'--value:' +
								percentage(choice.count, post.choicesCount).toString() +
								'; --size:2rem; --thickness: 3px;'}
						>
							{percentage(choice.count, post.choicesCount)}
						</div>
						{choice.name}
					</div>
				{:else}
					<p class="font-bold my-2">No choices yet!</p>
				{/each}
			{:else}
				{#each post.choices as choice, i}
					<button
						on:click={async () => {
							let res = await fetch(
								`${DEV_ORIGIN}/choices?name=${post.choices[i].name}&post_id=${post.id}&user_id=${$page.data.user?.sub}`,
								{
									credentials: 'include',
									method: 'POST'
								}
							);
							const choice = await res.json();
							post.hasVoted = true;
							res = await fetch(`${DEV_ORIGIN}/choices/count/${post.choices[i].post_id}`, {
								credentials: 'include'
							});
							const totalCount = await res.json();
							post.choicesCount = totalCount;

							for (let k = 0; k < post.choices.length; k++) {
								const res2 = await fetch(`${DEV_ORIGIN}/choices/${post.choices[k].name}/count`, {
									credentials: 'include'
								});
								const choiceCount = parseInt(await res2.text());
								post.choices[k].count = choiceCount;
							}
						}}
						class="btn btn-sm my-2">{choice.name}</button
					>
				{:else}
					<p class="font-bold my-2">No choices yet!</p>
				{/each}
			{/if}
			<!-- ends here -->
		</div>
		<div class="flex gap-5 text-sm">
			<p class="text-slate-500">submitted {timeSince(Date.parse(post.created_at))} ago</p>
		</div>
	</div>
</div>
