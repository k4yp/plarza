<script>
    import { onMount } from 'svelte';

    let posts = []

    const loadPosts = async () => {
        const response = await fetch('http://127.0.0.1:5000/posts')
        posts = await response.json()    
    }

    onMount(loadPosts)
</script>
  
<div class="posts">
    {#each posts as post}
    <div class="post">
        <a href="{post.link}" target="_blank">

        <div class="post-header">
            <a href="https://{post.source}.com/{post.user}" target="_blank" class="user">@{post.user}@{post.source}.com</a>
            <p>{new Date(post.date * 1000).toLocaleString('en-GB')}</p>
        </div>
        
        <p class="caption">{post.caption}</p>

        {#if post.media !== 'None'}
            <img src={post.media} class="media" alt="{post.source} media"/>
        {/if}
        
        </a>
    </div>
    {/each}
</div>

<style>
    .posts {
        color: #272727;
        font-size: 15px;
        font-family: "Chivo Mono", monospace;
    }
    .post {
        min-width: 400px;
        margin: auto;
        width: 25%;
        padding: 16px;
    }
    a {
        display: block;
        text-decoration: none;
        color: inherit;
    }
    p {
        margin: 0;
    }
    .user:hover {
        text-decoration: underline;
    }
    .post-header {
        font-weight: bold;
        display: flex;
        align-items: center;
    }
    .media {
        display: block;
        width: 100%;
    }
    .user {
        padding-right: 10px;
    }
    .caption{
        padding-top: 16px;
        padding-bottom: 16px;
        margin: 0;
    }
</style>