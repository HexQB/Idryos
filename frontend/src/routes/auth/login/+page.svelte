<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let email = '';
	let password = '';
	let loading = false;
	let error = '';

	async function handleLogin() {
		if (!email || !password) {
			error = 'Veuillez remplir tous les champs';
			return;
		}

		loading = true;
		error = '';

		try {
			const response = await fetch('http://localhost:8000/auth/login', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({ email, password }),
			});

			if (response.ok) {
				const data = await response.json();
				localStorage.setItem('access_token', data.access_token);
				localStorage.setItem('refresh_token', data.refresh_token);
				localStorage.setItem('user', JSON.stringify(data.user));
				goto('/dashboard');
			} else {
				const errorData = await response.json();
				error = errorData.error || 'Erreur de connexion';
			}
		} catch (err) {
			error = 'Erreur de connexion au serveur';
			console.error('Login error:', err);
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Connexion - Idryos</title>
</svelte:head>

<div class="min-h-[80vh] flex items-center justify-center">
	<div class="max-w-md w-full bg-white rounded-lg shadow-lg p-8">
		<div class="text-center mb-8">
			<h1 class="text-3xl font-bold text-gray-900">🔐 Connexion</h1>
			<p class="text-gray-600 mt-2">Connectez-vous à votre identité Idryos</p>
		</div>

		<form on:submit|preventDefault={handleLogin} class="space-y-6">
			{#if error}
				<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
					{error}
				</div>
			{/if}

			<div>
				<label for="email" class="block text-sm font-medium text-gray-700 mb-2">
					Email
				</label>
				<input
					type="email"
					id="email"
					bind:value={email}
					required
					class="input-field"
					placeholder="votre@email.com"
				/>
			</div>

			<div>
				<label for="password" class="block text-sm font-medium text-gray-700 mb-2">
					Mot de passe
				</label>
				<input
					type="password"
					id="password"
					bind:value={password}
					required
					class="input-field"
					placeholder="••••••••"
				/>
			</div>

			<button
				type="submit"
				disabled={loading}
				class="w-full btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
			>
				{loading ? 'Connexion...' : 'Se connecter'}
			</button>
		</form>

		<div class="mt-6 text-center">
			<p class="text-sm text-gray-600">
				Pas encore de compte ?
				<a href="/auth/register" class="text-indigo-600 hover:text-indigo-500 font-medium">
					Créer une identité
				</a>
			</p>
		</div>

		<div class="mt-8 pt-6 border-t border-gray-200">
			<div class="text-center">
				<p class="text-xs text-gray-500 mb-4">🔒 Connexion sécurisée et privée</p>
				<div class="grid grid-cols-3 gap-2 text-xs text-gray-400">
					<div>✅ Chiffrement bout en bout</div>
					<div>✅ Sans pistage</div>
					<div>✅ Auto-hébergé</div>
				</div>
			</div>
		</div>
	</div>
</div>
