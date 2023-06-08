///
//  Generated code. Do not modify.
//  source: sample_module.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class DummySchema extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DummySchema', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'sample_module'), createEmptyInstance: create)
    ..aOB(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'some')
    ..hasRequiredFields = false
  ;

  DummySchema._() : super();
  factory DummySchema({
    $core.bool? some,
  }) {
    final _result = create();
    if (some != null) {
      _result.some = some;
    }
    return _result;
  }
  factory DummySchema.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DummySchema.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DummySchema clone() => DummySchema()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DummySchema copyWith(void Function(DummySchema) updates) => super.copyWith((message) => updates(message as DummySchema)) as DummySchema; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DummySchema create() => DummySchema._();
  DummySchema createEmptyInstance() => create();
  static $pb.PbList<DummySchema> createRepeated() => $pb.PbList<DummySchema>();
  @$core.pragma('dart2js:noInline')
  static DummySchema getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DummySchema>(create);
  static DummySchema? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get some => $_getBF(0);
  @$pb.TagNumber(1)
  set some($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSome() => $_has(0);
  @$pb.TagNumber(1)
  void clearSome() => clearField(1);
}

