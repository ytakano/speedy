use std::io;
use std::mem;

use crate::context::Context;
use crate::endianness::Endianness;
use crate::writable::Writable;

pub trait Writer< C: Context > {
    fn write_bytes( &mut self, slice: &[u8] ) -> io::Result< () >;

    fn context( &self ) -> &C;
    fn context_mut( &mut self ) -> &mut C;

    #[inline(always)]
    fn can_write_at_least( &self, _size: usize ) -> Option< bool > {
        None
    }

    #[inline(always)]
    fn write_u8( &mut self, value: u8 ) -> io::Result< () > {
        let slice = unsafe { std::slice::from_raw_parts( &value, 1 ) };
        self.write_bytes( slice )
    }

    #[inline(always)]
    fn write_u16( &mut self, mut value: u16 ) -> io::Result< () > {
        self.context().endianness().swap_u16( &mut value );
        let slice = unsafe { std::slice::from_raw_parts( &value as *const u16 as *const u8, 2 ) };
        self.write_bytes( slice )
    }

    #[inline(always)]
    fn write_u32( &mut self, mut value: u32 ) -> io::Result< () > {
        self.context().endianness().swap_u32( &mut value );
        let slice = unsafe { std::slice::from_raw_parts( &value as *const u32 as *const u8, 4 ) };
        self.write_bytes( slice )
    }

    #[inline(always)]
    fn write_u64( &mut self, mut value: u64 ) -> io::Result< () > {
        self.context().endianness().swap_u64( &mut value );
        let slice = unsafe { std::slice::from_raw_parts( &value as *const u64 as *const u8, 8 ) };
        self.write_bytes( slice )
    }

    #[inline(always)]
    fn write_i8( &mut self, value: i8 ) -> io::Result< () > {
        self.write_u8( value as u8 )
    }

    #[inline(always)]
    fn write_i16( &mut self, value: i16 ) -> io::Result< () > {
        self.write_u16( value as u16 )
    }

    #[inline(always)]
    fn write_i32( &mut self, value: i32 ) -> io::Result< () > {
        self.write_u32( value as u32 )
    }

    #[inline(always)]
    fn write_i64( &mut self, value: i64 ) -> io::Result< () > {
        self.write_u64( value as u64 )
    }

    #[inline(always)]
    fn write_f32( &mut self, value: f32 ) -> io::Result< () > {
        let value: u32 = unsafe { mem::transmute( value ) };
        self.write_u32( value )
    }

    #[inline(always)]
    fn write_f64( &mut self, value: f64 ) -> io::Result< () > {
        let value: u64 = unsafe { mem::transmute( value ) };
        self.write_u64( value )
    }

    #[inline(always)]
    fn endianness( &self ) -> Endianness {
        self.context().endianness()
    }

    #[inline(always)]
    fn write_value< T: Writable< C > >( &mut self, item: &T ) -> io::Result< () > {
        item.write_to( self )
    }

    #[inline]
    fn write_slice< T >( &mut self, slice: &[T] ) -> io::Result< () >
        where T: Writable< C >
    {
        if T::speedy_is_primitive() && (mem::size_of::< T >() == 1 || !self.endianness().conversion_necessary()) {
            let bytes = unsafe { T::speedy_slice_as_bytes( slice ) };
            self.write_bytes( bytes )
        } else {
            for value in slice {
                self.write_value( value )?;
            }
            Ok(())
        }
    }

    #[inline]
    fn write_collection< T >( &mut self, collection: impl IntoIterator< Item = T > ) -> io::Result< () >
        where T: Writable< C >
    {
        for item in collection {
            item.write_to( self )?;
        }

        Ok(())
    }
}
