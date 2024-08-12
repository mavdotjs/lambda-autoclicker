<script lang="ts">
	import Tabs from '$lib/Tabs.svelte';
	import { Store } from 'tauri-plugin-store-api';
	import { open } from '@tauri-apps/plugin-shell';
	import Radio from '$lib/Radio.svelte';

	const clicking = $state(false);

	function fixedAnchor(a: HTMLAnchorElement) {
		a.onclick = async (e) => {
			e.preventDefault();
			await open(a.href);
		};
	}
	class SavedState {
		mode = $state<string>('random');
		options_for_fixed = $state({});
		options_for_random = $state<{ min: number; max: number }>({ min: 0, max: 0 });

		selected_count = $state<string>('forever');
		button = $state<string>('left');
		quantity = $state<string>('single');

		do_mouse_pos = $state(false);
		mouse_pos = $state<{ x: number, y: number }>({ x: 0, y: 0 });

		constructor() {
			$inspect(this.mode, this.options_for_fixed, this.options_for_random);
			let first = true;
			$effect(() => {
				if (first) {
					this.mode = localStorage.getItem('mode') || this.mode;
					const fixedLoaded = localStorage.getItem('fixed');
					this.options_for_fixed = fixedLoaded
						? JSON.parse(fixedLoaded)
						: this.options_for_fixed;
					const randomLoaded = localStorage.getItem('random');
					this.options_for_random = randomLoaded
						? JSON.parse(randomLoaded)
						: this.options_for_random;

					this.selected_count = localStorage.getItem('selected_count') || this.selected_count;
					this.button = localStorage.getItem('mouse_button') || this.button;
					this.quantity = localStorage.getItem('quantity') || this.quantity;

					const loaded_mouse_pos = localStorage.getItem('pos')
					this.mouse_pos = loaded_mouse_pos ? JSON.parse(loaded_mouse_pos) : this.mouse_pos
					const loaded_do_mouse_pos = localStorage.getItem('use_mouse_pos')
					this.do_mouse_pos = loaded_do_mouse_pos ? JSON.parse(loaded_do_mouse_pos) : this.do_mouse_pos

					first = false;
				}
				localStorage.setItem('mode', this.mode);
				localStorage.setItem('fixed', JSON.stringify(this.options_for_fixed));
				localStorage.setItem('random', JSON.stringify(this.options_for_random));

				localStorage.setItem('selected_count', this.selected_count)
				localStorage.setItem('mouse_button', this.button)
				localStorage.setItem('quantity', this.quantity)
				localStorage.setItem('pos', JSON.stringify(this.mouse_pos))

				localStorage.setItem('use_mouse_pos', JSON.stringify(this.do_mouse_pos))
			});
		}
	}
	let config = new SavedState();
</script>

<div class="bg-base-200 h-48 rounded-box">
	<span class="text-neutral uppercase font-appbartop ml-2"
		><span class="border-b">click interval</span></span
	>
	<Tabs bind:tab={config.mode}>
		{#snippet _random()}
			<div class="flex flex-col">
				<span class="text-xs"> Time between each click will be random number between: </span>
				<div class="join">
					<label class="input input-sm input-bordered items-center gap-2 join-item">
						min: <input type="number" min="0" bind:value={config.options_for_random.min} />ms
					</label>
					<label class="input input-sm input-bordered items-center gap-2 join-item">
						max: <input type="number" min="0" bind:value={config.options_for_random.max} />ms
					</label>
				</div>
			</div>
		{/snippet}
		{#snippet _fixed()}
			<span class="text-xs">Time between click</span>
		{/snippet}
	</Tabs>
</div>

<div class="bg-base-200 flex flex-row px-1 pr-2 mt-2 gap-x-2 h-10 items-center rounded-box">
	<span class="text-neutral uppercase font-appbartop ml-1 mb-1"
		><span class="border-b">mouse position</span></span
	>
	<input type="checkbox" bind:checked={config.do_mouse_pos} class="checkbox checkbox-primary">
	<div class="join">
		<label class="input input-xs input-bordered items-center gap-2 join-item">
			x <input type="number" min="0" bind:value={config.mouse_pos.x} />
		</label>
		<label class="input input-xs input-bordered items-center gap-2 join-item">
			y <input type="number" min="0" bind:value={config.mouse_pos.y} />
		</label>
	</div>
	<button class="btn btn-xs btn-primary flex-grow uppercase">pick</button>
</div>

<div class="grid grid-cols-2 gap-x-2 mt-2 h-28">
	<div class="flex flex-col pl-1 bg-base-200 rounded-box">
		<span class="text-neutral uppercase font-appbartop ml-1 mb-1"
			><span class="border-b">click type</span></span
		>
		<Tabs bind:tab={config.button}>
			{#snippet _left()}{/snippet}
			{#snippet _middle()}{/snippet}
			{#snippet _right()}{/snippet}
		</Tabs>
		<Tabs bind:tab={config.quantity}>
			{#snippet _single()}{/snippet}
			{#snippet _double()}{/snippet}
		</Tabs>
	</div>
	<div class="bg-base-200 pl-1 flex flex-col rounded-box">
		<span class="text-neutral uppercase font-appbartop ml-1 mb-1"
			><span class="border-b">repeat</span></span
		>
		<Radio bind:value={config.selected_count}>
			{#snippet _forever()}
				Until Stopped
			{/snippet}
			{#snippet _number(checked)}
				<input class="input input-xs input-bordered" type="number" min="0" disabled={!checked} /> Times
			{/snippet}
		</Radio>
	</div>
</div>

<div class="grid grid-cols-2 gap-4 h-16 mt-2 p-2 bg-base-200 rounded-box">
	<button class="btn btn-block btn-neutral">Start <kbd class="kbd">F6</kbd></button>
	<button class="btn btn-block btn-neutral">Keybinds</button>
</div>

<div class="flex flex-row h-8 bg-base-200 mt-2 items-center px-2 rounded-box">
	<div class="flex-grow flex">
		<a use:fixedAnchor href="https://github.com/mavdotjs/lambda-autoclicker">
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
	<span class="text-neutral uppercase font-appbartop ml-2"
		><span class="border-b">built by mavdotjs</span></span
	>
</div>
