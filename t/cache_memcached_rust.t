use Test2::V0 -no_srand => 1;
use Cache::Memcached::Rust;
use YAML ();

subtest 'basic, unconnected' => sub {

  my $cache = Cache::Memcached::Rust->new("memcache://127.0.0.1:12345?timeout=10&tcp_nodelay=true");
  isa_ok $cache, 'Cache::Memcached::Rust';

  note YAML::Dump($cache);

  # this should destroy
  undef $cache;

};

done_testing;


