<?hh
/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

namespace hack_ns2;

/**
 * Original thrift enum:-
 * Status
 */
<<\ThriftTypeInfo(shape('uri' => 'test.dev/hack/ns2/Status'))>>
enum Status: int {
  Unknown = 0;
}

class Status_TEnumStaticMetadata implements \IThriftEnumStaticMetadata {
  public static function getEnumMetadata()[]: \tmeta_ThriftEnum {
    return \tmeta_ThriftEnum::fromShape(
      shape(
        "name" => "module2_with_patch.Status",
        "elements" => dict[
          0 => "Unknown",
        ],
      )
    );
  }

  public static function getAllStructuredAnnotations()[write_props]: \TEnumAnnotations {
    return shape(
      'enum' => dict[],
      'constants' => dict[
      ],
    );
  }
}

/**
 * Original thrift struct:-
 * FBStruct
 */
<<\ThriftTypeInfo(shape('uri' => 'test.dev/hack/ns2/FBStruct'))>>
class FBStruct implements \IThriftSyncStruct, \IThriftStructMetadata {
  use \ThriftSerializationTrait;

  const \ThriftStructTypes::TSpec SPEC = dict[
    1 => shape(
      'var' => 'str_value',
      'type' => \TType::STRING,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'str_value' => 1,
  ];

  const type TConstructorShape = shape(
    ?'str_value' => ?string,
  );

  const int STRUCTURAL_ID = 7258556236863001185;
  /**
   * Original thrift field:-
   * 1: string str_value
   */
  public string $str_value;

  public function __construct(?string $str_value = null)[] {
    $this->str_value = $str_value ?? '';
  }

  public static function withDefaultValues()[]: this {
    return new static();
  }

  public static function fromShape(self::TConstructorShape $shape)[]: this {
    return new static(
      Shapes::idx($shape, 'str_value'),
    );
  }

  public function getName()[]: string {
    return 'FBStruct';
  }

  public static function getStructMetadata()[]: \tmeta_ThriftStruct {
    return \tmeta_ThriftStruct::fromShape(
      shape(
        "name" => "module2_with_patch.FBStruct",
        "fields" => vec[
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 1,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_primitive" => \tmeta_ThriftPrimitiveType::THRIFT_STRING_TYPE,
                )
              ),
              "name" => "str_value",
            )
          ),
        ],
        "is_union" => false,
      )
    );
  }

  public static function getAllStructuredAnnotations()[write_props]: \TStructAnnotations {
    return shape(
      'struct' => dict[
        '\facebook\thrift\op\GeneratePatch' => \facebook\thrift\op\GeneratePatch::fromShape(
          shape(
          )
        ),
      ],
      'fields' => dict[
      ],
    );
  }

  public function getInstanceKey()[write_props]: string {
    return \TCompactSerializer::serialize($this);
  }

}

/**
 * Original thrift struct:-
 * FBStructPatch
 */
<<\ThriftTypeInfo(shape('uri' => 'test.dev/hack/ns2/FBStructPatch'))>>
class FBStructPatch implements \IThriftSyncStruct, \IThriftStructMetadata, \IThriftStructWithClearTerseFields {
  use \ThriftSerializationTrait;

  const \ThriftStructTypes::TSpec SPEC = dict[
    1 => shape(
      'var' => 'assign',
      'type' => \TType::STRUCT,
      'class' => \hack_ns2\FBStruct::class,
    ),
    2 => shape(
      'var' => 'clear',
      'is_terse' => true,
      'type' => \TType::BOOL,
    ),
    3 => shape(
      'var' => 'patchPrior',
      'is_terse' => true,
      'type' => \TType::STRUCT,
      'class' => \hack_ns2\FBStructFieldPatch::class,
    ),
    5 => shape(
      'var' => 'ensure',
      'is_terse' => true,
      'type' => \TType::STRUCT,
      'class' => \hack_ns2\FBStructEnsureStruct::class,
    ),
    6 => shape(
      'var' => 'patch',
      'is_terse' => true,
      'type' => \TType::STRUCT,
      'class' => \hack_ns2\FBStructFieldPatch::class,
    ),
    7 => shape(
      'var' => 'remove',
      'is_terse' => true,
      'type' => \TType::LST,
      'etype' => \TType::I16,
      'elem' => shape(
        'type' => \TType::I16,
      ),
      'format' => 'collection',
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'assign' => 1,
    'clear' => 2,
    'patchPrior' => 3,
    'ensure' => 5,
    'patch' => 6,
    'remove' => 7,
  ];

  const type TConstructorShape = shape(
    ?'assign' => ?\hack_ns2\FBStruct,
    ?'clear' => ?bool,
    ?'patchPrior' => ?\hack_ns2\FBStructFieldPatch,
    ?'ensure' => ?\hack_ns2\FBStructEnsureStruct,
    ?'patch' => ?\hack_ns2\FBStructFieldPatch,
    ?'remove' => ?Vector<int>,
  );

  const int STRUCTURAL_ID = 4973839723579967743;
  /**
   * Assigns to a (set) value.
   * 
   * If set, all other operations are ignored.
   * 
   * Note: Optional and union fields must be set before assigned.
   * 
   * 
   * Original thrift field:-
   * 1: module2_with_patch.FBStruct assign
   */
  public ?\hack_ns2\FBStruct $assign;
  /**
   * Clears a value. Applies first.
   * 
   * Original thrift field:-
   * 2: bool clear
   */
  public bool $clear;
  /**
   * Patches any previously set values. Applies second.
   * 
   * Original thrift field:-
   * 3: module2_with_patch.FBStructFieldPatch patchPrior
   */
  public ?\hack_ns2\FBStructFieldPatch $patchPrior;
  /**
   * Initialize fields, using the given defaults. Applies third.
   * 
   * Original thrift field:-
   * 5: module2_with_patch.FBStructEnsureStruct ensure
   */
  public ?\hack_ns2\FBStructEnsureStruct $ensure;
  /**
   * Patches any set value, including newly set values. Applies last.
   * 
   * Original thrift field:-
   * 6: module2_with_patch.FBStructFieldPatch patch
   */
  public ?\hack_ns2\FBStructFieldPatch $patch;
  /**
   * Removes entries, if present. Applies third.
   * 
   * Original thrift field:-
   * 7: patch.FieldIdList remove
   */
  public Vector<int> $remove;

  public function __construct(?\hack_ns2\FBStruct $assign = null, ?bool $clear = null, ?\hack_ns2\FBStructFieldPatch $patchPrior = null, ?\hack_ns2\FBStructEnsureStruct $ensure = null, ?\hack_ns2\FBStructFieldPatch $patch = null, ?Vector<int> $remove = null)[] {
    $this->assign = $assign;
    $this->clear = $clear ?? false;
    $this->patchPrior = $patchPrior;
    $this->ensure = $ensure;
    $this->patch = $patch;
    $this->remove = $remove ?? Vector {};
  }

  public static function withDefaultValues()[]: this {
    return new static();
  }

  public static function fromShape(self::TConstructorShape $shape)[]: this {
    return new static(
      Shapes::idx($shape, 'assign'),
      Shapes::idx($shape, 'clear'),
      Shapes::idx($shape, 'patchPrior'),
      Shapes::idx($shape, 'ensure'),
      Shapes::idx($shape, 'patch'),
      Shapes::idx($shape, 'remove'),
    );
  }

  public function getName()[]: string {
    return 'FBStructPatch';
  }

  public function clearTerseFields()[write_props]: void {
    $this->clear = false;
    $this->patchPrior = null;
    $this->ensure = null;
    $this->patch = null;
    $this->remove = Vector {};
  }

  public static function getStructMetadata()[]: \tmeta_ThriftStruct {
    return \tmeta_ThriftStruct::fromShape(
      shape(
        "name" => "module2_with_patch.FBStructPatch",
        "fields" => vec[
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 1,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_struct" => \tmeta_ThriftStructType::fromShape(
                    shape(
                      "name" => "module2_with_patch.FBStruct",
                    )
                  ),
                )
              ),
              "name" => "assign",
              "is_optional" => true,
            )
          ),
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 2,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_primitive" => \tmeta_ThriftPrimitiveType::THRIFT_BOOL_TYPE,
                )
              ),
              "name" => "clear",
            )
          ),
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 3,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_struct" => \tmeta_ThriftStructType::fromShape(
                    shape(
                      "name" => "module2_with_patch.FBStructFieldPatch",
                    )
                  ),
                )
              ),
              "name" => "patchPrior",
            )
          ),
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 5,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_struct" => \tmeta_ThriftStructType::fromShape(
                    shape(
                      "name" => "module2_with_patch.FBStructEnsureStruct",
                    )
                  ),
                )
              ),
              "name" => "ensure",
            )
          ),
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 6,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_struct" => \tmeta_ThriftStructType::fromShape(
                    shape(
                      "name" => "module2_with_patch.FBStructFieldPatch",
                    )
                  ),
                )
              ),
              "name" => "patch",
            )
          ),
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 7,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_typedef" => \tmeta_ThriftTypedefType::fromShape(
                    shape(
                      "name" => "patch.FieldIdList",
                      "underlyingType" => \tmeta_ThriftType::fromShape(
                        shape(
                          "t_list" => \tmeta_ThriftListType::fromShape(
                            shape(
                              "valueType" => \tmeta_ThriftType::fromShape(
                                shape(
                                  "t_primitive" => \tmeta_ThriftPrimitiveType::THRIFT_I16_TYPE,
                                )
                              ),
                            )
                          ),
                        )
                      ),
                    )
                  ),
                )
              ),
              "name" => "remove",
            )
          ),
        ],
        "is_union" => false,
      )
    );
  }

  public static function getAllStructuredAnnotations()[write_props]: \TStructAnnotations {
    return shape(
      'struct' => dict[
        '\facebook\thrift\annotation\cpp\Adapter' => \facebook\thrift\annotation\cpp\Adapter::fromShape(
          shape(
            "name" => "::apache::thrift::op::detail::StructPatchAdapter<::test::hack::ns2::FBStructPatchStruct>",
            "underlyingName" => "FBStructPatchStruct",
            "extraNamespace" => "",
          )
        ),
      ],
      'fields' => dict[
        'assign' => shape(
          'field' => dict[
            '\facebook\thrift\annotation\Box' => \facebook\thrift\annotation\Box::fromShape(
              shape(
              )
            ),
          ],
          'type' => dict[
            '\facebook\thrift\op\GeneratePatch' => \facebook\thrift\op\GeneratePatch::fromShape(
              shape(
              )
            ),
          ],
        ),
        'patchPrior' => shape(
          'field' => dict[],
          'type' => dict[
            '\facebook\thrift\annotation\cpp\Adapter' => \facebook\thrift\annotation\cpp\Adapter::fromShape(
              shape(
                "name" => "::apache::thrift::op::detail::FieldPatchAdapter<::test::hack::ns2::FBStructFieldPatchStruct>",
                "underlyingName" => "FBStructFieldPatchStruct",
                "extraNamespace" => "",
              )
            ),
          ],
        ),
        'ensure' => shape(
          'field' => dict[],
          'type' => dict[
            '\facebook\thrift\annotation\cpp\Frozen2Exclude' => \facebook\thrift\annotation\cpp\Frozen2Exclude::fromShape(
              shape(
              )
            ),
          ],
        ),
        'patch' => shape(
          'field' => dict[],
          'type' => dict[
            '\facebook\thrift\annotation\cpp\Adapter' => \facebook\thrift\annotation\cpp\Adapter::fromShape(
              shape(
                "name" => "::apache::thrift::op::detail::FieldPatchAdapter<::test::hack::ns2::FBStructFieldPatchStruct>",
                "underlyingName" => "FBStructFieldPatchStruct",
                "extraNamespace" => "",
              )
            ),
          ],
        ),
        'remove' => shape(
          'field' => dict[],
          'type' => dict[
            '\facebook\thrift\annotation\cpp\Adapter' => \facebook\thrift\annotation\cpp\Adapter::fromShape(
              shape(
                "name" => "::apache::thrift::op::detail::FieldIdListToSetAdapter",
              )
            ),
          ],
        ),
      ],
    );
  }

  public function getInstanceKey()[write_props]: string {
    return \TCompactSerializer::serialize($this);
  }

}

/**
 * Original thrift struct:-
 * FBStructFieldPatch
 */
<<\ThriftTypeInfo(shape('uri' => 'test.dev/hack/ns2/FBStructFieldPatch'))>>
class FBStructFieldPatch implements \IThriftSyncStruct, \IThriftStructMetadata, \IThriftStructWithClearTerseFields {
  use \ThriftSerializationTrait;

  const \ThriftStructTypes::TSpec SPEC = dict[
    1 => shape(
      'var' => 'str_value',
      'is_terse' => true,
      'type' => \TType::STRUCT,
      'class' => \facebook\thrift\op\StringPatch::class,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'str_value' => 1,
  ];

  const type TConstructorShape = shape(
    ?'str_value' => ?\facebook\thrift\op\StringPatch,
  );

  const int STRUCTURAL_ID = 946063019501443060;
  /**
   * Original thrift field:-
   * 1: patch.StringPatch str_value
   */
  public ?\facebook\thrift\op\StringPatch $str_value;

  public function __construct(?\facebook\thrift\op\StringPatch $str_value = null)[] {
    $this->str_value = $str_value;
  }

  public static function withDefaultValues()[]: this {
    return new static();
  }

  public static function fromShape(self::TConstructorShape $shape)[]: this {
    return new static(
      Shapes::idx($shape, 'str_value'),
    );
  }

  public function getName()[]: string {
    return 'FBStructFieldPatch';
  }

  public function clearTerseFields()[write_props]: void {
    $this->str_value = null;
  }

  public static function getStructMetadata()[]: \tmeta_ThriftStruct {
    return \tmeta_ThriftStruct::fromShape(
      shape(
        "name" => "module2_with_patch.FBStructFieldPatch",
        "fields" => vec[
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 1,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_struct" => \tmeta_ThriftStructType::fromShape(
                    shape(
                      "name" => "patch.StringPatch",
                    )
                  ),
                )
              ),
              "name" => "str_value",
            )
          ),
        ],
        "is_union" => false,
      )
    );
  }

  public static function getAllStructuredAnnotations()[write_props]: \TStructAnnotations {
    return shape(
      'struct' => dict[
        '\facebook\thrift\annotation\cpp\Adapter' => \facebook\thrift\annotation\cpp\Adapter::fromShape(
          shape(
            "name" => "::apache::thrift::op::detail::FieldPatchAdapter<::test::hack::ns2::FBStructFieldPatchStruct>",
            "underlyingName" => "FBStructFieldPatchStruct",
            "extraNamespace" => "",
          )
        ),
      ],
      'fields' => dict[
        'str_value' => shape(
          'field' => dict[
            '\facebook\thrift\annotation\InternBox' => \facebook\thrift\annotation\InternBox::fromShape(
              shape(
              )
            ),
          ],
          'type' => dict[
            '\facebook\thrift\annotation\cpp\Adapter' => \facebook\thrift\annotation\cpp\Adapter::fromShape(
              shape(
                "name" => "::apache::thrift::op::detail::StringPatchAdapter<::apache::thrift::op::StringPatchStruct>",
                "underlyingName" => "StringPatchStruct",
              )
            ),
          ],
        ),
      ],
    );
  }

  public function getInstanceKey()[write_props]: string {
    return \TCompactSerializer::serialize($this);
  }

}

/**
 * Original thrift struct:-
 * FBStructEnsureStruct
 */
<<\ThriftTypeInfo(shape('uri' => 'test.dev/hack/ns2/FBStructEnsureStruct'))>>
class FBStructEnsureStruct implements \IThriftSyncStruct, \IThriftStructMetadata {
  use \ThriftSerializationTrait;

  const \ThriftStructTypes::TSpec SPEC = dict[
    1 => shape(
      'var' => 'str_value',
      'type' => \TType::STRING,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'str_value' => 1,
  ];

  const type TConstructorShape = shape(
    ?'str_value' => ?string,
  );

  const int STRUCTURAL_ID = 6613020566836874857;
  /**
   * Original thrift field:-
   * 1: string str_value
   */
  public ?string $str_value;

  public function __construct(?string $str_value = null)[] {
    $this->str_value = $str_value;
  }

  public static function withDefaultValues()[]: this {
    return new static();
  }

  public static function fromShape(self::TConstructorShape $shape)[]: this {
    return new static(
      Shapes::idx($shape, 'str_value'),
    );
  }

  public function getName()[]: string {
    return 'FBStructEnsureStruct';
  }

  public static function getStructMetadata()[]: \tmeta_ThriftStruct {
    return \tmeta_ThriftStruct::fromShape(
      shape(
        "name" => "module2_with_patch.FBStructEnsureStruct",
        "fields" => vec[
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 1,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_primitive" => \tmeta_ThriftPrimitiveType::THRIFT_STRING_TYPE,
                )
              ),
              "name" => "str_value",
              "is_optional" => true,
            )
          ),
        ],
        "is_union" => false,
      )
    );
  }

  public static function getAllStructuredAnnotations()[write_props]: \TStructAnnotations {
    return shape(
      'struct' => dict[
        '\facebook\thrift\annotation\cpp\Frozen2Exclude' => \facebook\thrift\annotation\cpp\Frozen2Exclude::fromShape(
          shape(
          )
        ),
      ],
      'fields' => dict[
      ],
    );
  }

  public function getInstanceKey()[write_props]: string {
    return \TCompactSerializer::serialize($this);
  }

}

/**
 * Original thrift struct:-
 * FBStructSafePatch
 */
<<\ThriftTypeInfo(shape('uri' => 'test.dev/hack/ns2/FBStructSafePatch'))>>
class FBStructSafePatch implements \IThriftSyncStruct, \IThriftStructMetadata, \IThriftStructWithClearTerseFields {
  use \ThriftSerializationTrait;

  const \ThriftStructTypes::TSpec SPEC = dict[
    1 => shape(
      'var' => 'version',
      'is_terse' => true,
      'type' => \TType::I32,
    ),
    2 => shape(
      'var' => 'data',
      'is_terse' => true,
      'type' => \TType::STRING,
      'is_binary' => true,
    ),
  ];
  const dict<string, int> FIELDMAP = dict[
    'version' => 1,
    'data' => 2,
  ];

  const type TConstructorShape = shape(
    ?'version' => ?int,
    ?'data' => ?string,
  );

  const int STRUCTURAL_ID = 3942627235385260502;
  /**
   * Original thrift field:-
   * 1: i32 version
   */
  public int $version;
  /**
   * Original thrift field:-
   * 2: binary data
   */
  public string $data;

  public function __construct(?int $version = null, ?string $data = null)[] {
    $this->version = $version ?? 0;
    $this->data = $data ?? '';
  }

  public static function withDefaultValues()[]: this {
    return new static();
  }

  public static function fromShape(self::TConstructorShape $shape)[]: this {
    return new static(
      Shapes::idx($shape, 'version'),
      Shapes::idx($shape, 'data'),
    );
  }

  public function getName()[]: string {
    return 'FBStructSafePatch';
  }

  public function clearTerseFields()[write_props]: void {
    $this->version = 0;
    $this->data = '';
  }

  public static function getStructMetadata()[]: \tmeta_ThriftStruct {
    return \tmeta_ThriftStruct::fromShape(
      shape(
        "name" => "module2_with_patch.FBStructSafePatch",
        "fields" => vec[
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 1,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_primitive" => \tmeta_ThriftPrimitiveType::THRIFT_I32_TYPE,
                )
              ),
              "name" => "version",
            )
          ),
          \tmeta_ThriftField::fromShape(
            shape(
              "id" => 2,
              "type" => \tmeta_ThriftType::fromShape(
                shape(
                  "t_primitive" => \tmeta_ThriftPrimitiveType::THRIFT_BINARY_TYPE,
                )
              ),
              "name" => "data",
            )
          ),
        ],
        "is_union" => false,
      )
    );
  }

  public static function getAllStructuredAnnotations()[write_props]: \TStructAnnotations {
    return shape(
      'struct' => dict[
        '\facebook\thrift\annotation\cpp\Frozen2Exclude' => \facebook\thrift\annotation\cpp\Frozen2Exclude::fromShape(
          shape(
          )
        ),
      ],
      'fields' => dict[
      ],
    );
  }

  public function getInstanceKey()[write_props]: string {
    return \TCompactSerializer::serialize($this);
  }

}

