<script lang="ts">
    import {useLocation, useNavigate} from "svelte-navigator";
    import {onMount} from "svelte";
    import {getCookie} from "../cookies";
    import {user} from "../stores";
    import {mafiaHost} from "../variables";

    const navigate = useNavigate();
    const location = useLocation();

    let name;

    async function onLoginSubmit() {
        const res = await fetch(`${mafiaHost}/register`, {
            method: 'POST',
            mode: 'cors',
            credentials: 'include',
            cache: 'no-cache',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name,
            })
        })
        const {guid} = await res.json()

        $user = {name, guid}
        const from = ($location.state && $location.state.from) || "/components";
        navigate(from, {replace: true});
    }

    async function onAppLoad() {
        const guidC = getCookie('mafia-guid')

        if (guidC) {
            let res = await fetch(`${mafiaHost}/user/${guidC}`, {
                method: 'GET',
                mode: 'cors',
                credentials: 'include',
                cache: 'no-cache'
            })

            if (res.status == 200) {
                const {name} = await res.json()

                $user = {name, guid: guidC}
                const from = ($location.state && $location.state.from) || "/components";
                navigate(from, {replace: true});
            }
        }
    }

    onMount(onAppLoad)
</script>

<form on:submit|preventDefault={onLoginSubmit}>
    <input bind:value={name} name="name" placeholder="John Doe" type="text"/>
    <br/>
    <button type="submit">Join Game</button>
</form>
