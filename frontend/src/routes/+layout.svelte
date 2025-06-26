<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	let mounted = false;

	onMount(() => {
		mounted = true;
	});
</script>

<svelte:head>
	<title>Idryos - Privacy-Centric Identity Platform</title>
	<meta name="description" content="Plateforme de gestion d'identité ultra-privée, décentralisée, et open source" />
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
	<nav class="bg-white shadow-sm border-b">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="flex justify-between h-16">
				<div class="flex items-center">
					<div class="flex-shrink-0">
						<h1 class="text-2xl font-bold text-indigo-600">🚀 Idryos</h1>
					</div>
					<div class="hidden md:ml-8 md:flex md:space-x-8">
						<a href="/" class="text-gray-900 hover:text-indigo-600 px-3 py-2 text-sm font-medium">
							Accueil
						</a>
						<a href="/auth/login" class="text-gray-500 hover:text-indigo-600 px-3 py-2 text-sm font-medium">
							Connexion
						</a>
						<a href="/auth/register" class="text-gray-500 hover:text-indigo-600 px-3 py-2 text-sm font-medium">
							Inscription
						</a>
					</div>
				</div>
			</div>
		</div>
	</nav>

	<main class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
		<slot />
	</main>

	<footer class="bg-white border-t mt-12">
		<div class="max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8">
			<p class="text-center text-sm text-gray-500">
				© 2025 Idryos - Privacy-Centric Decentralized Identity Platform
			</p>
		</div>
	</footer>
</div>
