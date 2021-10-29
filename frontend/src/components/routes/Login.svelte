<script lang="ts">
    import {useLocation, useNavigate} from "svelte-navigator";
    import {onMount} from "svelte";
    import {user} from "../../stores";
    import {getCookie} from "../../cookies";
    import backend from "../../backend";

    const navigate = useNavigate();
    const location = useLocation();

    let name;


    const onLoginSubmit = async () => {
        const {guid} = await backend.register(name)

        $user = {name, guid}
        const from = ($location.state && $location.state.from) || "/game";
        navigate(from, {replace: true});
    }

    const onAppLoad = async () => {
        const guidC = getCookie('mafia-guid')

        if (guidC) {
            const res = await backend.user(guidC)

            if (res) {
                const {name} = res
                $user = {name, guid: guidC}
                const from = ($location.state && $location.state.from) || "/game";
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
