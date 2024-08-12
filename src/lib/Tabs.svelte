<script lang="ts">
	import type { Snippet } from 'svelte';

	let { tab = $bindable(), ...props }: {
        tab: string | undefined;
        [x: `_${string}`]: Snippet<[]>;
	} = $props();
	const tabs = Object.fromEntries(Object.entries(props).filter((v) => v[0].startsWith('_')).map(([name, snippet]): [string, Snippet<[]>] => [name.replace('_', ''), snippet]));
</script>

<div role="tablist" class="tabs tabs-boxed">
	{#each Object.entries(tabs) as [tabID]}
		<!-- svelte-ignore a11y_interactive_supports_focus -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_missing_attribute -->
		<a role="tab" class="tab uppercase" onclick={() => tab = tabID} class:tab-active={tabID == tab}>{tabID}</a>
	{/each}
</div>

{#if tab}
    {@render tabs[tab]()}
{/if}