<script>
    import { onMount } from 'svelte';

    let posts = [];

    const loadPosts = async () => {
        try {
            const data = {
                user_id: 1
            };
            const response = await fetch('http://172.20.0.4:8080/posts', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(data)
            });
        posts = await response.json();
        } catch (error) {
            console.error(error);
        }
    };
  
    onMount(loadPosts);
  </script>
  
<div class="container">
    <div class="posts">
        {#each posts as post}
        <div class="post">
            <a href={post.link} target="_blank">
                <div class="post-header">
                <a href={`https://${post.source}/${post.username}`} target="_blank" class="user">@{post.username}@{post.source}</a>
                </div>
    
                <div class="content">
                {#if post.media_url !== null}
                    <div class="media-container">
                    <img src={`http://0.0.0.0:8080/media/${post.post_id}`} class="media" alt="{post.source} media"/>
                    </div>
                {/if}
    
                <div class="post-content">
                    <div class="caption">{post.caption}</div>
                    <p>{new Date(post.date * 1000).toLocaleString('en-GB')}</p>
                </div>
                </div>
            </a>
            </div>
        {/each}
    </div>
</div>  

<style>
    .container {
        width: 50%;
        margin: auto;
    }

    .posts {
        color: #272727;
        font-size: 15px;
        font-family: "Chivo Mono", monospace;
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
        grid-gap: 16px;
    }

    .post {
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

    .content {
        display: flex;
    }

    .media-container {
        padding-top: 16px;
        width: 100px;
        height: 100px;
        margin-right: 16px;
    }

    .media {
        display: block;
        width: 100px;
        height: 100px;
        object-fit: cover;
        border-radius: 32px;
        border: solid 2px #000000;
    }

    .post-content {
        flex-grow: 1;
    }

    .caption {
        padding-top: 16px;
        padding-bottom: 16px;
        margin: 0;
    }

    @media (max-width: 600px) {
        .container {
            width: 100%;
        }
    }
</style>

