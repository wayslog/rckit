use clap::ArgMatches;
use std::collections::HashMap;
use std::fmt;
use std::slice::SliceConcatExt;

pub struct ReporterConfig {
    pub node: String,
    pub output: String,
    pub column: String,
    pub format: String,
}

impl ReporterConfig {
    pub fn new<'a>(args: &ArgMatches<'a>) -> ReporterConfig {
        let node = args.value_of("node").expect("must get node").to_string();
        let output = args
            .value_of("output")
            .expect("must get default output")
            .to_string();
        let column = args
            .value_of("column")
            .expect("must get default column")
            .to_string();
        let format = args
            .value_of("format")
            .expect("must get default format")
            .to_string();
        ReporterConfig {
            node,
            output,
            column,
            format,
        }
    }

    pub fn build(self) -> Reporter {
        Reporter {
            cfg: self,
        }
    }
}

/// Repoter is the struct which collect all cluster infos and output the infomation
/// the process is :
///
///     init cluster by node -> send info all to each nodes -> compute all inneed info -> output them
///
/// make sure that reporter can only run in one single thread.
pub struct Reporter {
    pub cfg: ReporterConfig,
}

/// the output option may be follows:
///
/// | option name | redis config      | redis commnad | type                    | comment                       |
/// | ----------- | ----------------- | ------------- | ----------------------- | ----------------------------- |
/// | version     | redis_version     | info all      | string                  |                               |
/// | model       | redis_model       | info all      | string                  | cluster/standalone            |
/// | pid         | process_id        | info all      | u64                     |                               |
/// | addr        | -                 | cluster nodes | string                  | line.split()[1].split("@")[0] |
/// | conns       | connected_clients | info all      | u64                     |                               |
/// | used        | used_memory       | info all      | u64                     |                               |
/// | max         | maxmemory         | info all      | u64                     |                               |
/// | hit         | keyspace_hits     | info all      | u64                     |                               |
/// | miss        | keyspace_misses   | info all      | u64                     |                               |
/// | role        | role              | info all      | String                  | master or slave               |
/// | cmds        | cmdstat_*         | Info all      | HashMap<String, CmdStat>|                               |
/// | keys        | db*:keys          | info all      | u64                     |                               |
/// | slots       | -                 | cluster nodes | Vec<Slot>               | parse redis comand            |
/// |             |                   |               |                         |                               |
#[derive(Debug)]
pub struct Measurement {
    version: String,
    model: String,
    pid: u64,
    addr: String,
    conns: u64,
    used: u64,
    max: u64,
    hit: u64,
    miss: u64,
    role: String,
    cmds: HashMap<String, CmdStat>,
    keys: u64,
    slots: Slots,
}

#[derive(Debug)]
pub struct CmdStat {
    calls: f64,
    userc: f64,
    userc_per_call: f64,
}

#[derive(Debug)]
pub struct Slots(Vec<Slot>);

impl fmt::Display for Slots{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let list: Vec<_> = self.0.iter().map(|x| format!("{}", x)).collect();
        write!(f, "{}", list.join(" "))
    }
}

#[derive(Debug)]
pub struct Slot {
    begin: usize,
    end: usize,
}

impl fmt::Display for Slot{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.begin == self.end {
            write!(f, "{}", self.begin)
        } else {
            write!(f, "{}-{}", self.begin, self.end)
        }
    }
}

/*
# Server
redis_version:5.0.2
redis_git_sha1:00000000
redis_git_dirty:0
redis_build_id:6fc0f5745dcba2c3
redis_mode:standalone
os:Darwin 18.2.0 x86_64
arch_bits:64
multiplexing_api:kqueue
atomicvar_api:atomic-builtin
gcc_version:4.2.1
process_id:711
run_id:e1d45f4c21cb658a460ffdde0b5b0c2fef591332
tcp_port:6379
uptime_in_seconds:706820
uptime_in_days:8
hz:10
configured_hz:10
lru_clock:2405880
executable:/usr/local/opt/redis/bin/redis-server
config_file:/usr/local/etc/redis.conf

# Clients
connected_clients:2
client_recent_max_input_buffer:4
client_recent_max_output_buffer:0
blocked_clients:0

# Memory
used_memory:210121648
used_memory_human:200.39M
used_memory_rss:1839104
used_memory_rss_human:1.75M
used_memory_peak:210301616
used_memory_peak_human:200.56M
used_memory_peak_perc:99.91%
used_memory_overhead:97248792
used_memory_startup:988032
used_memory_dataset:112872856
used_memory_dataset_perc:53.97%
allocator_allocated:210069440
allocator_active:1797120
allocator_resident:1797120
total_system_memory:17179869184
total_system_memory_human:16.00G
used_memory_lua:41984
used_memory_lua_human:41.00K
used_memory_scripts:152
used_memory_scripts_human:152B
number_of_cached_scripts:1
maxmemory:0
maxmemory_human:0B
maxmemory_policy:noeviction
allocator_frag_ratio:0.01
allocator_frag_bytes:18446744073501279296
allocator_rss_ratio:1.00
allocator_rss_bytes:0
rss_overhead_ratio:1.02
rss_overhead_bytes:41984
mem_fragmentation_ratio:0.01
mem_fragmentation_bytes:18446744073501321280
mem_not_counted_for_evict:0
mem_replication_backlog:0
mem_clients_slaves:0
mem_clients_normal:66600
mem_aof_buffer:0
mem_allocator:libc
active_defrag_running:0
lazyfree_pending_objects:0

# Persistence
loading:0
rdb_changes_since_last_save:0
rdb_bgsave_in_progress:0
rdb_last_save_time:1545386214
rdb_last_bgsave_status:ok
rdb_last_bgsave_time_sec:1
rdb_current_bgsave_time_sec:-1
rdb_last_cow_size:0
aof_enabled:0
aof_rewrite_in_progress:0
aof_rewrite_scheduled:0
aof_last_rewrite_time_sec:-1
aof_current_rewrite_time_sec:-1
aof_last_bgrewrite_status:ok
aof_last_write_status:ok
aof_last_cow_size:0

# Stats
total_connections_received:344
total_commands_processed:24095
instantaneous_ops_per_sec:0
total_net_input_bytes:1330083
total_net_output_bytes:1366315
instantaneous_input_kbps:0.01
instantaneous_output_kbps:1.69
rejected_connections:0
sync_full:0
sync_partial_ok:0
sync_partial_err:0
expired_keys:0
expired_stale_perc:0.00
expired_time_cap_reached_count:0
evicted_keys:0
keyspace_hits:166
keyspace_misses:10
pubsub_channels:0
pubsub_patterns:0
latest_fork_usec:701
migrate_cached_sockets:0
slave_expires_tracked_keys:0
active_defrag_hits:0
active_defrag_misses:0
active_defrag_key_hits:0
active_defrag_key_misses:0

# Replication
role:master
connected_slaves:0
master_replid:2e95d61890892cf3ea3deb13872b29a0f2341fc8
master_replid2:0000000000000000000000000000000000000000
master_repl_offset:0
second_repl_offset:-1
repl_backlog_active:0
repl_backlog_size:1048576
repl_backlog_first_byte_offset:0
repl_backlog_histlen:0

# CPU
used_cpu_sys:62.994048
used_cpu_user:42.910709
used_cpu_sys_children:1.509271
used_cpu_user_children:8.053970

# Commandstats
cmdstat_slowlog:calls=252,usec=833,usec_per_call=3.31
cmdstat_get:calls=39,usec=62,usec_per_call=1.59
cmdstat_scan:calls=1135,usec=8515,usec_per_call=7.50
cmdstat_type:calls=76,usec=114,usec_per_call=1.50
cmdstat_eval:calls=5,usec=1447,usec_per_call=289.40
cmdstat_scard:calls=5,usec=9,usec_per_call=1.80
cmdstat_llen:calls=10,usec=11,usec_per_call=1.10
cmdstat_select:calls=221,usec=215,usec_per_call=0.97
cmdstat_ping:calls=3,usec=0,usec_per_call=0.00
cmdstat_config:calls=151,usec=16511,usec_per_call=109.34
cmdstat_zadd:calls=5,usec=44,usec_per_call=8.80
cmdstat_auth:calls=20,usec=43,usec_per_call=2.15
cmdstat_debug:calls=10,usec=2438554,usec_per_call=243855.41
cmdstat_set:calls=10282,usec=13806,usec_per_call=1.34
cmdstat_hlen:calls=5,usec=1,usec_per_call=0.20
cmdstat_setex:calls=305,usec=603,usec_per_call=1.98
cmdstat_info:calls=128,usec=16636,usec_per_call=129.97
cmdstat_lpush:calls=249,usec=755,usec_per_call=3.03
cmdstat_del:calls=10841,usec=11766,usec_per_call=1.09
cmdstat_strlen:calls=36,usec=52,usec_per_call=1.44
cmdstat_zcard:calls=5,usec=3,usec_per_call=0.60
cmdstat_sadd:calls=127,usec=333,usec_per_call=2.62
cmdstat_mset:calls=2,usec=92,usec_per_call=46.00
cmdstat_pfadd:calls=5,usec=14,usec_per_call=2.80
cmdstat_command:calls=1,usec=1377,usec_per_call=1377.00
cmdstat_hset:calls=5,usec=28,usec_per_call=5.60
cmdstat_pfcount:calls=41,usec=138,usec_per_call=3.37
cmdstat_latency:calls=131,usec=485,usec_per_call=3.70

# Cluster
cluster_enabled:0

# Keyspace
db0:keys=1985419,expires=0,avg_ttl=0
 */
