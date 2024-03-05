import React from "react";
import FetchStatusCheck from "./FetchStatusCheck";
import { Table, TableBody, TableCell, TableContainer, TableHead, TableRow } from "@mui/material";
import type { AccountNftInfo, ListAccountNftResponse } from "@tariproject/typescript-bindings/wallet-daemon-client";
import type { apiError } from "../api/helpers/types";
import { DataTableCell } from "./StyledComponents";
import { renderJson } from "../utils/helpers";
import { IoCheckmarkOutline, IoCloseOutline } from "react-icons/io5";

function NftsList({ metadata, is_burned }: AccountNftInfo) {
  return (
    <TableRow>
      <DataTableCell>{metadata.name || <i>No name</i>}</DataTableCell>
      <DataTableCell>
        {metadata.image_url ? (
          <a href={metadata.image_url} target="_blank" rel="noopener noreferrer">
            <img src={metadata.image_url} style={{ maxWidth: "100px", maxHeight: "100px", objectFit: "contain" }} />
          </a>
        ) : (
          <i>No image</i>
        )}
      </DataTableCell>
      <DataTableCell>{renderJson(metadata)}</DataTableCell>
      <DataTableCell>
        {is_burned ? (
          <IoCheckmarkOutline style={{ height: 22, width: 22 }} color="#DB7E7E" />
        ) : (
          <IoCloseOutline style={{ height: 22, width: 22 }} color="#5F9C91" />
        )}
      </DataTableCell>
    </TableRow>
  );
}

export default function NFTList({
  nftsListIsError,
  nftsListIsFetching,
  nftsListError,
  nftsListData,
}: {
  nftsListIsError: boolean;
  nftsListIsFetching: boolean;
  nftsListError: apiError | null;
  nftsListData?: ListAccountNftResponse;
}) {
  if (nftsListIsError || nftsListIsFetching) {
    <FetchStatusCheck
      isError={nftsListIsError}
      errorMessage={nftsListError?.message || "Error fetching data"}
      isLoading={nftsListIsFetching}
    />;
  }
  return (
    <TableContainer>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Name</TableCell>
            <TableCell>Image</TableCell>
            <TableCell>Metadata</TableCell>
            <TableCell style={{ whiteSpace: "nowrap" }}>Is Burned</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {nftsListData?.nfts.map(({ metadata, is_burned }: AccountNftInfo, index) => (
            <NftsList key={index} metadata={metadata} is_burned={is_burned} />
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
