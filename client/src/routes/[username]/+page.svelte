<script>
    import { onMount } from 'svelte'
    import { page } from '$app/stores'
    const username = $page.params.username

    let user = {};

    const loadPosts = async () => {
        const response = await fetch(`http://127.0.0.1:5000/user/${username}`)
        const data = await response.json()
        user = data[0]
    }
    
    onMount(loadPosts)
</script>

<div class="profile">
    {#if user.media !== null}
        <img src={user.media} class="profile-picture" alt="{user.username} profile"/>
    {:else}
        <img src="https://www.gravatar.com/avatar/{user.date}?d=identicon" class="profile-picture" alt="{user.username} profile"/>
    {/if}

    {#if user.display !== null}
        <p class="display">{user.display}</p>
    {/if}
    
    <p class="username" >@{user.username}</p>

    {#if user.bio !== null}
        <p class = "bio">{user.bio}</p>
    {/if}
</div>

<style>
.profile{
    font-family: "Courier Prime", monospace;
    padding-top: 64px;
    margin-left: 50%;
}
.profile-picture {
    background-color: brown;
    width: 64px;
    height: 64px;
    border-radius: 50%;
    object-fit: cover;
}
.display{
    font-weight: bold;
    font-size: 20px;
    margin: 0;
}
.username{
    margin: 0;
}
.bio{

}
</style>