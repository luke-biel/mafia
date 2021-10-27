<script lang="ts">
    import {Route, Router} from "svelte-navigator";
    import PrivateRoute from "./utils/PrivateRoute.svelte";
    import Login from "./routes/Login.svelte";
    import Game from "./routes/Game.svelte";
</script>

<Router>
    <header>
        Mafia
    </header>

    <main>
        <Route path="/">
            <Login/>
        </Route>

        <PrivateRoute let:location path="/game">
            <Game/>
        </PrivateRoute>
    </main>
</Router>

<style>
    header {
        font-size: 2em
    }
</style>
