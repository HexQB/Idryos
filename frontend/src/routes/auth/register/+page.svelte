<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let username = '';
	let email = '';
	let password = '';
	let confirmPassword = '';
	let loading = false;
	let error = '';

	async function handleRegister() {
		if (!username || !email || !password || !confirmPassword) {
			error = 'Veuillez remplir tous les champs';
			return;
		}

		if (password !== confirmPassword) {
			error = 'Les mots de passe ne correspondent pas';
			return;
		}

		if (password.length < 8) {
			error = 'Le mot de passe doit contenir au moins 8 caractères';
			return;
		}

		loading = true;
		error = '';

		try {
			const response = await fetch('http://localhost:8000/auth/register', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({ username, email, password }),
			});

			if (response.ok) {
				const data = await response.json();
				// Rediriger vers la page de connexion avec un message de succès
				goto('/auth/login?message=Compte créé avec succès !');
			} else {
				const errorData = await response.json();
				error = errorData.error || 'Erreur lors de la création du compte';
			}
		} catch (err) {
			error = 'Erreur de connexion au serveur';
			console.error('Register error:', err);
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Inscription - Idryos</title>
</svelte:head>

<div class="min-h-[80vh] flex items-center justify-center">
	<div class="max-w-md w-full bg-white rounded-lg shadow-lg p-8">
		<div class="text-center mb-8">
			<h1 class="text-3xl font-bold text-gray-900">🚀 Créer une identité</h1>
			<p class="text-gray-600 mt-2">Rejoignez la révolution de l'identité décentralisée</p>
		</div>

		<form on:submit|preventDefault={handleRegister} class="space-y-6">
			{#if error}
				<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
					{error}
				</div>
			{/if}

			<div>
				<label for="username" class="block text-sm font-medium text-gray-700 mb-2">
					Nom d'utilisateur
				</label>
				<input
					type="text"
					id="username"
					bind:value={username}
					required
					class="input-field"
					placeholder="votre_nom"
				/>
			</div>

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
				<p class="text-xs text-gray-500 mt-1">Minimum 8 caractères</p>
			</div>

			<div>
				<label for="confirmPassword" class="block text-sm font-medium text-gray-700 mb-2">
					Confirmer le mot de passe
				</label>
				<input
					type="password"
					id="confirmPassword"
					bind:value={confirmPassword}
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
				{loading ? 'Création...' : 'Créer mon identité'}
			</button>
		</form>

		<div class="mt-6 text-center">
			<p class="text-sm text-gray-600">
				Déjà un compte ?
				<a href="/auth/login" class="text-indigo-600 hover:text-indigo-500 font-medium">
					Se connecter
				</a>
			</p>
		</div>

		<div class="mt-8 pt-6 border-t border-gray-200">
			<div class="text-center">
				<p class="text-xs text-gray-500 mb-4">🛡️ Vos données restent privées</p>
				<div class="grid grid-cols-2 gap-4 text-xs text-gray-400">
					<div class="text-left">
						<div class="font-medium text-gray-600 mb-1">✅ Garanties</div>
						<div>• Pas de pistage</div>
						<div>• Stockage local</div>
						<div>• Chiffrement bout en bout</div>
					</div>
					<div class="text-left">
						<div class="font-medium text-gray-600 mb-1">🔑 Fonctionnalités</div>
						<div>• Identité décentralisée</div>
						<div>• OAuth2/OpenID</div>
						<div>• Support DID</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
