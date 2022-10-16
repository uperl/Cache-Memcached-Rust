package Cache::Memcached::Rust {

  use warnings;
  use 5.020;
  use experimental qw( postderef signatures );
  use FFI::Platypus 2.00;

# ABSTRACT: Perl client for memcached, in Rust language

  my $ffi = FFI::Platypus->new( api => 2, lang => 'Rust' );
  $ffi->bundle;
  $ffi->mangler(sub ($name) { "mc_$name" });
  $ffi->type("object(@{[ __PACKAGE__ ]})" => 'mc');

  $ffi->attach( new => ['string'] => 'u64' => sub ($xsub, $class, $url) {
    my $index = $xsub->($url);
    bless \$index, $class;
  });

  $ffi->attach( DESTROY => ['mc']);
}

1;
