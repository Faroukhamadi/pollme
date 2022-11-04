<script lang="ts">
	import timeSince from '$lib/utils/timeSince';
	import type { Post } from '../../routes/+page';

	export let post: Post;
</script>

<div class="flex gap-5 bg-indigo-300 rounded-md m-4 p-4">
	<div class="flex flex-col justify-center">
		<button
			on:click={(e) => {
				// posts[0].votes += 1;
				post.votes = (parseInt(post.votes) + 1).toString();
				console.log('upvote');
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				aria-hidden="true"
				class="h-7 w-7 text-neutral-500 hover:rounded-sm hover:bg-neutral-700 hover:bg-neutral-600 hover:text-green-400"
				><path
					fill-rule="evenodd"
					d="M11.47 7.72a.75.75 0 011.06 0l7.5 7.5a.75.75 0 11-1.06 1.06L12 9.31l-6.97 6.97a.75.75 0 01-1.06-1.06l7.5-7.5z"
					clip-rule="evenodd"
				/></svg
			>
		</button>
		<p class="text-center">{post.votes}</p>
		<button>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				aria-hidden="true"
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
		<h3 class="text-xl">{post.title.slice(50)}</h3>
		<div>
			<button class="btn btn-sm">{post.first_choice}</button>
			<button class="btn btn-sm">{post.second_choice}</button>
		</div>
		<div class="flex gap-5 text-sm">
			<p>{post.choice_count} votes</p>
			<p class="text-slate-500">submitted {timeSince(Date.parse(post.created_at))} ago</p>
		</div>
	</div>
</div>
