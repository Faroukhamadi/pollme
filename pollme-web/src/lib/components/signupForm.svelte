<script lang="ts">
	import { enhance } from '$app/forms';
	let passwordConfirmValid = true;
</script>

<form
	method="POST"
	use:enhance={({ cancel, data }) => {
		const password = data.get('password');
		const passwordConfirm = data.get('password-confirm');

		if (password !== passwordConfirm) {
			cancel();
			passwordConfirmValid = false;
		} else {
			passwordConfirmValid = true;
		}
	}}
>
	<div class="card-body">
		<div class="form-control">
			<label for="username" class="label">
				<span class="label-text">Username</span>
			</label>
			<input
				autocomplete="username"
				id="username"
				placeholder="username"
				name="username"
				type="text"
				class="input input-bordered"
			/>
		</div>
		<div class="form-control">
			<label for="password" class="label">
				<span class="label-text">Password</span>
			</label>
			<input
				autocomplete="current-password"
				id="password"
				placeholder="password"
				name="password"
				type="password"
				class="input input-bordered"
			/>
		</div>
		<div class="form-control">
			<label for="password-confirm" class="label">
				<span class="label-text">Password</span>
			</label>
			<input
				autocomplete="current-password"
				id="password-confirm"
				placeholder="confirm password"
				name="password-confirm"
				type="password"
				class="input input-bordered"
				class:input-error={!passwordConfirmValid}
			/>
			{#if !passwordConfirmValid}
				<span class="label label-text text-error m-0">Passwords do not match</span>
				<!-- <div class="alert alert-error shadow-lg mt-4">
					<div>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="stroke-current flex-shrink-0 h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>Error! passwords don't match</span>
					</div>
				</div> -->
			{/if}
		</div>
		<div class="form-control mt-6" />
		<div class="flex flex-col w-full border-opacity-50">
			<button class="btn btn-primary">Signup</button>
			<a data-sveltekit-prefetch href="/login" class="mt-4 link link-primary"
				>Already have an account?</a
			>
		</div>
	</div>
</form>
