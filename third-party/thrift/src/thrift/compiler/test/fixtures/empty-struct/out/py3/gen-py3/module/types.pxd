#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from libc.stdint cimport (
    int8_t as cint8_t,
    int16_t as cint16_t,
    int32_t as cint32_t,
    int64_t as cint64_t,
    uint32_t as cuint32_t,
)
from libcpp.string cimport string
from libcpp cimport bool as cbool, nullptr, nullptr_t
from cpython cimport bool as pbool
from libcpp.memory cimport shared_ptr, unique_ptr
from libcpp.utility cimport move as cmove
from libcpp.vector cimport vector
from libcpp.set cimport set as cset
from libcpp.map cimport map as cmap, pair as cpair
from thrift.py3.exceptions cimport cTException
cimport folly.iobuf as _fbthrift_iobuf
cimport thrift.py3.exceptions
cimport thrift.py3.types
from thrift.py3.types cimport (
    bstring,
    bytes_to_string,
    field_ref as __field_ref,
    optional_field_ref as __optional_field_ref,
    required_field_ref as __required_field_ref,
    terse_field_ref as __terse_field_ref,
    union_field_ref as __union_field_ref,
    get_union_field_value as __get_union_field_value,
)
from thrift.py3.common cimport (
    RpcOptions as __RpcOptions,
    cThriftMetadata as __fbthrift_cThriftMetadata,
    MetadataBox as __MetadataBox,
)
from thrift.python.protocol cimport Protocol as __Protocol
from folly.optional cimport cOptional as __cOptional

cimport module.types_fields as _fbthrift_types_fields

cdef extern from "thrift/compiler/test/fixtures/empty-struct/gen-py3/module/types.h":
  pass





cdef extern from "thrift/compiler/test/fixtures/empty-struct/gen-cpp2/module_metadata.h" namespace "apache::thrift::detail::md":
    cdef cppclass ExceptionMetadata[T]:
        @staticmethod
        void gen(__fbthrift_cThriftMetadata &metadata)
cdef extern from "thrift/compiler/test/fixtures/empty-struct/gen-cpp2/module_metadata.h" namespace "apache::thrift::detail::md":
    cdef cppclass StructMetadata[T]:
        @staticmethod
        void gen(__fbthrift_cThriftMetadata &metadata)
cdef extern from "thrift/compiler/test/fixtures/empty-struct/gen-cpp2/module_types_custom_protocol.h" namespace "::cpp2":

    cdef cppclass cEmpty "::cpp2::Empty":
        cEmpty() except +
        cEmpty(const cEmpty&) except +
        bint operator==(cEmpty&)
        bint operator!=(cEmpty&)
        bint operator<(cEmpty&)
        bint operator>(cEmpty&)
        bint operator<=(cEmpty&)
        bint operator>=(cEmpty&)

    cdef enum cNada__type "::cpp2::Nada::Type":
        cNada__type___EMPTY__ "::cpp2::Nada::Type::__EMPTY__",

    cdef cppclass cNada "::cpp2::Nada":
        cNada() except +
        cNada(const cNada&) except +
        bint operator==(cNada&)
        bint operator!=(cNada&)
        bint operator<(cNada&)
        bint operator>(cNada&)
        bint operator<=(cNada&)
        bint operator>=(cNada&)
        cNada__type getType() const




cdef class Empty(thrift.py3.types.Struct):
    cdef shared_ptr[cEmpty] _cpp_obj
    cdef _fbthrift_types_fields.__Empty_FieldsSetter _fields_setter

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cEmpty])

cdef class __NadaType(thrift.py3.types.CompiledEnum):
    pass




cdef class Nada(thrift.py3.types.Union):
    cdef shared_ptr[cNada] _cpp_obj
    cdef readonly __NadaType type
    cdef readonly object value
    cdef _load_cache(Nada self)

    @staticmethod
    cdef unique_ptr[cNada] _make_instance(
        cNada* base_instance
    ) except *

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cNada])



