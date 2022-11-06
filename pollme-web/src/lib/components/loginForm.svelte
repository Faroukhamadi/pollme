<form method="POST">
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
		<div class="form-control mt-6" />
		<div class="flex flex-col w-full border-opacity-50">
			<button class="btn btn-primary">Login</button>
			<div class="divider">OR</div>
			<a class="btn btn-secondary" href="/signup">Signup</a>
		</div>
	</div>
</form>
