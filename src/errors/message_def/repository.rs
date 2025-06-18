use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_repository_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    // リポジトリ共通エラー
    let mut no_page_specified = HashMap::new();
    no_page_specified.insert("en", "No page specified");
    no_page_specified.insert("jp", "ページが指定されていません");
    map.insert(ErrorKey::NoPageSpecified, no_page_specified);

    let mut no_page_size_specified = HashMap::new();
    no_page_size_specified.insert("en", "No page size specified");
    no_page_size_specified.insert("jp", "ページサイズが指定されていません");
    map.insert(ErrorKey::NoPageSizeSpecified, no_page_size_specified);

    let mut invalid_pagination = HashMap::new();
    invalid_pagination.insert("en", "Invalid pagination");
    invalid_pagination.insert("jp", "無効なページング");
    map.insert(ErrorKey::InvalidPagination, invalid_pagination);

    let mut page_size_too_large = HashMap::new();
    page_size_too_large.insert("en", "Page size too large");
    page_size_too_large.insert("jp", "ページサイズが大きすぎます");
    map.insert(ErrorKey::PageSizeTooLarge, page_size_too_large);

    let mut no_data_found_in_pagination = HashMap::new();
    no_data_found_in_pagination.insert("en", "No data found in pagination");
    no_data_found_in_pagination.insert("jp", "ページングにデータが見つかりません");
    map.insert(
        ErrorKey::NoDataFoundInPagination,
        no_data_found_in_pagination,
    );
}
