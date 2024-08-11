<script lang="ts">
	import Tabs from '$lib/Tabs.svelte';
	import { Store } from 'tauri-plugin-store-api';
	import { appDataDir, join } from '@tauri-apps/api/path';
	import { onMount } from 'svelte';

	let store: Store;

	class SavedState {
		mode = $state<string>('random');
		options_for_fixed = $state({});
		options_for_random = $state<{ min: number; max: number }>({ min: 0, max: 0 });
		constructor() {
			$inspect(this.mode, this.options_for_fixed, this.options_for_random);
			let first = true;
			$effect(() => {
				if (first) {
					appState.mode = localStorage.getItem('mode') || appState.mode;
					const fixedLoaded = localStorage.getItem('fixed');
					appState.options_for_fixed = fixedLoaded
						? JSON.parse(fixedLoaded)
						: appState.options_for_fixed;
					const randomLoaded = localStorage.getItem('random');
					appState.options_for_random = randomLoaded
						? JSON.parse(randomLoaded)
						: appState.options_for_random;
					first = false;
				}
				localStorage.setItem('mode', this.mode);
				localStorage.setItem('fixed', JSON.stringify(this.options_for_fixed));
				localStorage.setItem('random', JSON.stringify(this.options_for_random));
			});
		}
	}
	let appState = new SavedState();
</script>

<div class="bg-base-200 h-48">
	<Tabs bind:tab={appState.mode} map={{ random: 'random interval', fixed: 'fixed interval' }}>
		{#snippet _random()}
			Random value between 2 millisecond values
			<div class="join">
				<label class="input input-bordered flex items-center gap-2 join-item">
					min
					<input type="number" min="0" bind:value={appState.options_for_random.min} />
					ms
				</label>
				<label class="input input-bordered flex items-center gap-2 join-item">
					max
					<input type="number" min="0" bind:value={appState.options_for_random.max} />
					ms
				</label>
			</div>
		{/snippet}
		{#snippet _fixed()}
			Fixed interval from time value
		{/snippet}
	</Tabs>
</div>

<div class="flex flex-row gap-x-2 h-48 mt-2">
	<div class="bg-base-200 flex-grow"></div>
	<div class="bg-base-200 flex-grow"></div>
</div>

<div class="flex flex-row gap-x-2 h-32 mt-2">
	<div class="bg-base-200 flex-grow"></div>
	<div class="bg-base-200 flex-grow"></div>
</div>

<div class="flex flex-row h-8 bg-base-200 mt-2 items-center px-2">
    <a href="https://github.com/mavdotjs/lambda-clicker">
        <svg viewBox="0 0 96 96" class="size-6" xmlns="http://www.w3.org/2000/svg"
		><path
			fill-rule="evenodd"
			clip-rule="evenodd"
			d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z"
			fill="#fff"
		/></svg
	>
    </a>
</div>
