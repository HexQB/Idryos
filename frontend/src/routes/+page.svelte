<script lang="ts">
	import { onMount } from 'svelte';

	let mounted = false;

	onMount(() => {
		mounted = true;
	});
</script>

<div class="text-center">
	<div class="max-w-4xl mx-auto">
		<h1 class="text-4xl font-bold text-gray-900 sm:text-6xl mb-6">
			🚀 <span class="text-indigo-600">Idryos</span>
		</h1>
		<p class="text-xl text-gray-600 mb-8">
			Plateforme de gestion d'identité ultra-privée, décentralisée, et open source
		</p>
		
		<div class="bg-white rounded-lg shadow-lg p-8 mb-12">
			<h2 class="text-2xl font-semibold text-gray-900 mb-6">🌟 Notre Mission</h2>
			<p class="text-lg text-gray-600 mb-6">
				Redonner le contrôle des identités numériques aux utilisateurs en proposant une solution auto-hébergeable, sans centralisation ni pistage.
			</p>
			
			<div class="grid md:grid-cols-2 gap-6 mt-8">
				<div class="text-left">
					<h3 class="text-lg font-semibold text-gray-900 mb-4">✅ Fonctionnalités</h3>
					<ul class="space-y-2 text-gray-600">
						<li>• Fournisseur OAuth2 / OpenID Connect</li>
						<li>• Gestion d'identités auto-hébergées</li>
						<li>• Support des DID (Decentralized Identifiers)</li>
						<li>• Communication P2P via libp2p</li>
						<li>• Stockage local ou décentralisé</li>
					</ul>
				</div>
				<div class="text-left">
					<h3 class="text-lg font-semibold text-gray-900 mb-4">🔒 Sécurité</h3>
					<ul class="space-y-2 text-gray-600">
						<li>• Pas de stockage non chiffré</li>
						<li>• Aucune collecte centralisée</li>
						<li>• Communications chiffrées bout en bout</li>
						<li>• Clés privées stockées localement</li>
					</ul>
				</div>
			</div>
		</div>

		<div class="flex justify-center space-x-4">
			<a 
				href="/auth/register" 
				class="bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-3 px-6 rounded-lg transition-colors"
			>
				Créer une identité
			</a>
			<a 
				href="/auth/login" 
				class="bg-white hover:bg-gray-50 text-indigo-600 font-medium py-3 px-6 rounded-lg border border-indigo-600 transition-colors"
			>
				Se connecter
			</a>
		</div>

		{#if mounted}
			<div class="mt-12 bg-white rounded-lg shadow-lg p-6">
				<h3 class="text-lg font-semibold text-gray-900 mb-4">📊 État des services</h3>
				<div class="grid md:grid-cols-3 gap-4">
					<div class="text-center p-4 bg-green-50 rounded-lg">
						<div class="text-green-600 font-semibold">Auth Service</div>
						<div class="text-sm text-green-600">🟢 En ligne</div>
					</div>
					<div class="text-center p-4 bg-green-50 rounded-lg">
						<div class="text-green-600 font-semibold">P2P Network</div>
						<div class="text-sm text-green-600">🟢 En ligne</div>
					</div>
					<div class="text-center p-4 bg-yellow-50 rounded-lg">
						<div class="text-yellow-600 font-semibold">IPFS</div>
						<div class="text-sm text-yellow-600">🟡 Optionnel</div>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>
