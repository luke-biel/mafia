<script>
    import {Route} from "svelte-navigator";
    import PrivateRouteGuard from "./PrivateRouteGuard.svelte";

    export let path;
</script>

<Route let:location let:navigate let:params {path}>
    <PrivateRouteGuard>
        <slot {location} {navigate} {params}/>
    </PrivateRouteGuard>
</Route>
