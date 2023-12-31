use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn conversion(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();

    // e.g. frclib_units_macros::unit_conversion!(meter f64, Feet f64, meter_to_feet);
    //this would mean meter -> Feet

    let mut iter = input.into_iter().filter(|token| {
        !matches!(
            token,
            proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Group(_)
        )
    });
    let from_name =
        syn::parse2::<syn::Ident>(iter.next().expect("could not find from ident").into())
            .expect("could not parse from ident as an ident");
    let from_inner_type =
        syn::parse2::<syn::Ident>(iter.next().expect("could not find from type ident").into())
            .expect("could not parse from type ident as an ident");
    let to_name = syn::parse2::<syn::Ident>(iter.next().expect("could not find to ident").into())
        .expect("could not parse to ident as an ident");
    let to_inner_type =
        syn::parse2::<syn::Ident>(iter.next().expect("could not find to type ident").into())
            .expect("could not parse to type ident as an ident");
    let conv_func =
        syn::parse2::<syn::Ident>(iter.next().expect("could not find third ident").into())
            .expect("could not parse third ident as an ident");

    let inv_conv_ident = syn::Ident::new(
        &format!("inverse_{}", conv_func),
        proc_macro2::Span::call_site(),
    );

    //create an inverse conv_func
    let inv_conv_func_block = quote! {
        fn #inv_conv_ident(value: #to_inner_type) -> #from_inner_type {
            (value / #conv_func(#from_inner_type::from(1.0)) as #to_inner_type) as #from_inner_type
        }
    };

    let impl_from_block = quote! {
        impl From<#from_name> for #to_name {
            fn from(value: #from_name) -> Self {
                #to_name(#conv_func(value.0))
            }
        }
        #[cfg(feature = "ref-ops")]
        impl From<&#from_name> for #to_name {
            fn from(value: &#from_name) -> Self {
                #to_name(#conv_func(value.0))
            }
        }
        impl From<#to_name> for #from_name {
            fn from(value: #to_name) -> Self {
                #from_name(#inv_conv_ident(value.0))
            }
        }
        #[cfg(feature = "ref-ops")]
        impl From<&#to_name> for #from_name {
            fn from(value: &#to_name) -> Self {
                #from_name(#inv_conv_ident(value.0))
            }
        }
    };

    //add math between the two types
    let impl_to_op_from_block = quote! {
        impl std::ops::Add<#to_name> for #from_name {
            type Output = #from_name;
            fn add(self, rhs: #to_name) -> Self::Output {
                self + #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Add<&#to_name> for #from_name {
            type Output = #from_name;
            fn add(self, rhs: &#to_name) -> Self::Output {
                self + #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Add<#to_name> for &#from_name {
            type Output = #from_name;
            fn add(self, rhs: #to_name) -> Self::Output {
                self + #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Add<&#to_name> for &#from_name {
            type Output = #from_name;
            fn add(self, rhs: &#to_name) -> Self::Output {
                self + #from_name::from(rhs)
            }
        }
        impl std::ops::AddAssign<#to_name> for #from_name {
            fn add_assign(&mut self, rhs: #to_name) {
                *self += #from_name::from(rhs);
            }
        }
        impl std::ops::Sub<#to_name> for #from_name {
            type Output = #from_name;
            fn sub(self, rhs: #to_name) -> Self::Output {
                self - #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Sub<&#to_name> for #from_name {
            type Output = #from_name;
            fn sub(self, rhs: &#to_name) -> Self::Output {
                self - #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Sub<#to_name> for &#from_name {
            type Output = #from_name;
            fn sub(self, rhs: #to_name) -> Self::Output {
                self - #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Sub<&#to_name> for &#from_name {
            type Output = #from_name;
            fn sub(self, rhs: &#to_name) -> Self::Output {
                self - #from_name::from(rhs)
            }
        }
        impl std::ops::SubAssign<#to_name> for #from_name {
            fn sub_assign(&mut self, rhs: #to_name) {
                *self -= #from_name::from(rhs);
            }
        }
        impl std::ops::Mul<#to_name> for #from_name {
            type Output = #from_name;
            fn mul(self, rhs: #to_name) -> Self::Output {
                self * #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Mul<&#to_name> for #from_name {
            type Output = #from_name;
            fn mul(self, rhs: &#to_name) -> Self::Output {
                self * #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Mul<#to_name> for &#from_name {
            type Output = #from_name;
            fn mul(self, rhs: #to_name) -> Self::Output {
                self * #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Mul<&#to_name> for &#from_name {
            type Output = #from_name;
            fn mul(self, rhs: &#to_name) -> Self::Output {
                self * #from_name::from(rhs)
            }
        }
        impl std::ops::MulAssign<#to_name> for #from_name {
            fn mul_assign(&mut self, rhs: #to_name) {
                *self *= #from_name::from(rhs);
            }
        }
        impl std::ops::Div<#to_name> for #from_name {
            type Output = #from_name;
            fn div(self, rhs: #to_name) -> Self::Output {
                self / #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Div<&#to_name> for #from_name {
            type Output = #from_name;
            fn div(self, rhs: &#to_name) -> Self::Output {
                self / #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Div<#to_name> for &#from_name {
            type Output = #from_name;
            fn div(self, rhs: #to_name) -> Self::Output {
                self / #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Div<&#to_name> for &#from_name {
            type Output = #from_name;
            fn div(self, rhs: &#to_name) -> Self::Output {
                self / #from_name::from(rhs)
            }
        }
        impl std::ops::DivAssign<#to_name> for #from_name {
            fn div_assign(&mut self, rhs: #to_name) {
                *self /= #from_name::from(rhs);
            }
        }
        impl std::ops::Rem<#to_name> for #from_name {
            type Output = #from_name;
            fn rem(self, rhs: #to_name) -> Self::Output {
                self % #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Rem<&#to_name> for #from_name {
            type Output = #from_name;
            fn rem(self, rhs: &#to_name) -> Self::Output {
                self % #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Rem<#to_name> for &#from_name {
            type Output = #from_name;
            fn rem(self, rhs: #to_name) -> Self::Output {
                self % #from_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Rem<&#to_name> for &#from_name {
            type Output = #from_name;
            fn rem(self, rhs: &#to_name) -> Self::Output {
                self % #from_name::from(rhs)
            }
        }
        impl std::ops::RemAssign<#to_name> for #from_name {
            fn rem_assign(&mut self, rhs: #to_name) {
                *self %= #from_name::from(rhs);
            }
        }
    };
    let impl_from_op_to_block = quote! {
        impl std::ops::Add<#from_name> for #to_name {
            type Output = #to_name;
            fn add(self, rhs: #from_name) -> Self::Output {
                self + #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Add<&#from_name> for #to_name {
            type Output = #to_name;
            fn add(self, rhs: &#from_name) -> Self::Output {
                self + #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Add<#from_name> for &#to_name {
            type Output = #to_name;
            fn add(self, rhs: #from_name) -> Self::Output {
                self + #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Add<&#from_name> for &#to_name {
            type Output = #to_name;
            fn add(self, rhs: &#from_name) -> Self::Output {
                self + #to_name::from(rhs)
            }
        }
        impl std::ops::AddAssign<#from_name> for #to_name {
            fn add_assign(&mut self, rhs: #from_name) {
                *self += #to_name::from(rhs);
            }
        }
        impl std::ops::Sub<#from_name> for #to_name {
            type Output = #to_name;
            fn sub(self, rhs: #from_name) -> Self::Output {
                self - #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Sub<&#from_name> for #to_name {
            type Output = #to_name;
            fn sub(self, rhs: &#from_name) -> Self::Output {
                self - #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Sub<#from_name> for &#to_name {
            type Output = #to_name;
            fn sub(self, rhs: #from_name) -> Self::Output {
                self - #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Sub<&#from_name> for &#to_name {
            type Output = #to_name;
            fn sub(self, rhs: &#from_name) -> Self::Output {
                self - #to_name::from(rhs)
            }
        }
        impl std::ops::SubAssign<#from_name> for #to_name {
            fn sub_assign(&mut self, rhs: #from_name) {
                *self -= #to_name::from(rhs);
            }
        }
        impl std::ops::Mul<#from_name> for #to_name {
            type Output = #to_name;
            fn mul(self, rhs: #from_name) -> Self::Output {
                self * #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Mul<&#from_name> for #to_name {
            type Output = #to_name;
            fn mul(self, rhs: &#from_name) -> Self::Output {
                self * #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Mul<#from_name> for &#to_name {
            type Output = #to_name;
            fn mul(self, rhs: #from_name) -> Self::Output {
                self * #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Mul<&#from_name> for &#to_name {
            type Output = #to_name;
            fn mul(self, rhs: &#from_name) -> Self::Output {
                self * #to_name::from(rhs)
            }
        }
        impl std::ops::MulAssign<#from_name> for #to_name {
            fn mul_assign(&mut self, rhs: #from_name) {
                *self *= #to_name::from(rhs);
            }
        }
        impl std::ops::Div<#from_name> for #to_name {
            type Output = #to_name;
            fn div(self, rhs: #from_name) -> Self::Output {
                self / #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Div<&#from_name> for #to_name {
            type Output = #to_name;
            fn div(self, rhs: &#from_name) -> Self::Output {
                self / #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Div<#from_name> for &#to_name {
            type Output = #to_name;
            fn div(self, rhs: #from_name) -> Self::Output {
                self / #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Div<&#from_name> for &#to_name {
            type Output = #to_name;
            fn div(self, rhs: &#from_name) -> Self::Output {
                self / #to_name::from(rhs)
            }
        }
        impl std::ops::DivAssign<#from_name> for #to_name {
            fn div_assign(&mut self, rhs: #from_name) {
                *self /= #to_name::from(rhs);
            }
        }
        impl std::ops::Rem<#from_name> for #to_name {
            type Output = #to_name;
            fn rem(self, rhs: #from_name) -> Self::Output {
                self % #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Rem<&#from_name> for #to_name {
            type Output = #to_name;
            fn rem(self, rhs: &#from_name) -> Self::Output {
                self % #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Rem<#from_name> for &#to_name {
            type Output = #to_name;
            fn rem(self, rhs: #from_name) -> Self::Output {
                self % #to_name::from(rhs)
            }
        }
        #[cfg(feature = "ref-ops")]
        impl std::ops::Rem<&#from_name> for &#to_name {
            type Output = #to_name;
            fn rem(self, rhs: &#from_name) -> Self::Output {
                self % #to_name::from(rhs)
            }
        }
        impl std::ops::RemAssign<#from_name> for #to_name {
            fn rem_assign(&mut self, rhs: #from_name) {
                *self %= #to_name::from(rhs);
            }
        }
    };

    //implement partial eq and partial ord between the two types
    let impl_partial_eq_ord_block = quote! {
        impl std::cmp::PartialEq<#to_name> for #from_name {
            fn eq(&self, other: &#to_name) -> bool {
                self.0 == #inv_conv_ident(other.0) as #from_inner_type
            }
        }
        impl std::cmp::PartialEq<#from_name> for #to_name {
            fn eq(&self, other: &#from_name) -> bool {
                self.0 == #conv_func(other.0) as #to_inner_type
            }
        }
        impl std::cmp::PartialOrd<#to_name> for #from_name {
            fn partial_cmp(&self, other: &#to_name) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&#inv_conv_ident(other.0))
            }
        }
        impl std::cmp::PartialOrd<#from_name> for #to_name {
            fn partial_cmp(&self, other: &#from_name) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&#conv_func(other.0))
            }
        }
    };

    output.extend(inv_conv_func_block);
    output.extend(impl_from_block);
    output.extend(impl_to_op_from_block);
    output.extend(impl_from_op_to_block);
    output.extend(impl_partial_eq_ord_block);

    output
}
