///
//  Generated code. Do not modify.
//  source: messages/basic.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class SampleNumber extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SampleNumber', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'basic'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'value', $pb.PbFieldType.O3)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dummyOne', $pb.PbFieldType.O3)
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dummyTwo', $pb.PbFieldType.O3)
    ..p<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dummyThree', $pb.PbFieldType.K3)
    ..hasRequiredFields = false
  ;

  SampleNumber._() : super();
  factory SampleNumber({
    $core.int? value,
    $core.int? dummyOne,
    $core.int? dummyTwo,
    $core.Iterable<$core.int>? dummyThree,
  }) {
    final _result = create();
    if (value != null) {
      _result.value = value;
    }
    if (dummyOne != null) {
      _result.dummyOne = dummyOne;
    }
    if (dummyTwo != null) {
      _result.dummyTwo = dummyTwo;
    }
    if (dummyThree != null) {
      _result.dummyThree.addAll(dummyThree);
    }
    return _result;
  }
  factory SampleNumber.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SampleNumber.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SampleNumber clone() => SampleNumber()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SampleNumber copyWith(void Function(SampleNumber) updates) => super.copyWith((message) => updates(message as SampleNumber)) as SampleNumber; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SampleNumber create() => SampleNumber._();
  SampleNumber createEmptyInstance() => create();
  static $pb.PbList<SampleNumber> createRepeated() => $pb.PbList<SampleNumber>();
  @$core.pragma('dart2js:noInline')
  static SampleNumber getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SampleNumber>(create);
  static SampleNumber? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get value => $_getIZ(0);
  @$pb.TagNumber(1)
  set value($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasValue() => $_has(0);
  @$pb.TagNumber(1)
  void clearValue() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get dummyOne => $_getIZ(1);
  @$pb.TagNumber(2)
  set dummyOne($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasDummyOne() => $_has(1);
  @$pb.TagNumber(2)
  void clearDummyOne() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get dummyTwo => $_getIZ(2);
  @$pb.TagNumber(3)
  set dummyTwo($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDummyTwo() => $_has(2);
  @$pb.TagNumber(3)
  void clearDummyTwo() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get dummyThree => $_getList(3);
}

class SampleLetter extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SampleLetter', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'basic'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'letter')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dummyOne', $pb.PbFieldType.O3)
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dummyTwo', $pb.PbFieldType.O3)
    ..p<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'dummyThree', $pb.PbFieldType.K3)
    ..hasRequiredFields = false
  ;

  SampleLetter._() : super();
  factory SampleLetter({
    $core.String? letter,
    $core.int? dummyOne,
    $core.int? dummyTwo,
    $core.Iterable<$core.int>? dummyThree,
  }) {
    final _result = create();
    if (letter != null) {
      _result.letter = letter;
    }
    if (dummyOne != null) {
      _result.dummyOne = dummyOne;
    }
    if (dummyTwo != null) {
      _result.dummyTwo = dummyTwo;
    }
    if (dummyThree != null) {
      _result.dummyThree.addAll(dummyThree);
    }
    return _result;
  }
  factory SampleLetter.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SampleLetter.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SampleLetter clone() => SampleLetter()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SampleLetter copyWith(void Function(SampleLetter) updates) => super.copyWith((message) => updates(message as SampleLetter)) as SampleLetter; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SampleLetter create() => SampleLetter._();
  SampleLetter createEmptyInstance() => create();
  static $pb.PbList<SampleLetter> createRepeated() => $pb.PbList<SampleLetter>();
  @$core.pragma('dart2js:noInline')
  static SampleLetter getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SampleLetter>(create);
  static SampleLetter? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get letter => $_getSZ(0);
  @$pb.TagNumber(1)
  set letter($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLetter() => $_has(0);
  @$pb.TagNumber(1)
  void clearLetter() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get dummyOne => $_getIZ(1);
  @$pb.TagNumber(2)
  set dummyOne($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasDummyOne() => $_has(1);
  @$pb.TagNumber(2)
  void clearDummyOne() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get dummyTwo => $_getIZ(2);
  @$pb.TagNumber(3)
  set dummyTwo($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDummyTwo() => $_has(2);
  @$pb.TagNumber(3)
  void clearDummyTwo() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get dummyThree => $_getList(3);
}

