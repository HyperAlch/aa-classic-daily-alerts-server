create table if not exists webhook 
(
    guild_id varchar not null,
    hook_url text not null
);

create unique index webhook_guild_idx on webhook (guild_id);