'use-client';
import { ChevronLeftIcon, ChevronRightIcon } from '@heroicons/react/20/solid';
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  useReactTable,
} from '@tanstack/react-table';
import { Dispatch, SetStateAction, useEffect, useState } from 'react';
import { Skeleton } from '../Skeleton/Skeleton';

type TableProps = {
  columns: ColumnDef<any>[];
  data: any;
  isCommon?: boolean;
  setCurrentPage?: Dispatch<SetStateAction<number>>;
  setPageLimit?: Dispatch<SetStateAction<number>>;
  isLoading?: boolean;
  setIsLoading?: Dispatch<SetStateAction<boolean>>;

  currentPage?: number;
  limitPage?: number;
  totalPage?: number;
};
const Table = ({
  columns,
  data,
  isCommon,
  setCurrentPage,
  setPageLimit,
  currentPage,
  totalPage,
  limitPage,
  isLoading,
  setIsLoading,
}: TableProps) => {
  const table = useReactTable({
    columns,
    data,

    // Pipeline
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getPaginationRowModel: getPaginationRowModel(),

    //
    debugTable: true,
  });

  const [isClient, setIsClient] = useState(false);
  // const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (!isCommon) {
      table.setPageSize(15);
    }
    setIsClient(true);
    setTimeout(() => {
      if (setIsLoading) setIsLoading(false);
    }, 1000);
  }, []);

  return (
    <>
      {isClient && (
        <div className=" overflow-hidden flex flex-col justify-between  bg-transparent w-full min-h-auto ">
          <table>
            <thead>
              {table.getHeaderGroups().map((headerGroup) => (
                <tr key={headerGroup.id}>
                  {headerGroup.headers.map((header) => {
                    return (
                      <th
                        className="p-5"
                        key={header.id}
                        colSpan={header.colSpan}
                      >
                        {header.isPlaceholder ? null : (
                          <div className="text-left font-medium">
                            {flexRender(
                              header.column.columnDef.header,
                              header.getContext(),
                            )}
                          </div>
                        )}
                      </th>
                    );
                  })}
                </tr>
              ))}
            </thead>
            {!isLoading ? ( // Check if loading is true
              <tbody>
                {table.getRowModel().rows.map((row) => {
                  return (
                    <tr
                      className="border border-1 border-zinc-300 border-x-0"
                      key={row.id}
                    >
                      {row.getVisibleCells().map((cell) => {
                        return (
                          <td
                            className="px-5 py-2  text-[16px] font-medium leading-normal"
                            key={cell.id}
                          >
                            {flexRender(
                              cell.column.columnDef.cell,
                              cell.getContext(),
                            )}
                          </td>
                        );
                      })}
                    </tr>
                  );
                })}
              </tbody>
            ) : (
              // If loading is false, render skeleton list
              Array.from({ length: 10 }).map((_, index) => (
                <tr
                  key={index}
                  className="border border-1 border-zinc-300 border-x-0"
                >
                  <td className="px-5 py-2  text-[16px] font-medium leading-normal">
                    <Skeleton.List />
                  </td>
                  <td className="px-5 py-2  text-[16px] font-medium leading-normal">
                    <Skeleton.List />
                  </td>
                  <td className="px-5 py-2  text-[16px] font-medium leading-normal">
                    <Skeleton.List />
                  </td>
                </tr>
              ))
            )}
          </table>
          {isLoading === false && data.length === 0 && (
            <div className="grid w-full h-full place-items-center">
              <p className=" text-center py-6 ">Data not found!</p>
            </div>
          )}
          {isCommon && isCommon === true && (
            <div className="flex flex-row justify-end items-center gap-2 p-5 ">
              {/* <div className="text-neutral-400 text-[12px] font-semibold uppercase tracking-wider">
                ROWS PER PAGE:
                <select
                  value={table.getState().pagination.pageSize}
                  className="bg-transparent text-primary"
                  onChange={(e) => {
                    table.setPageSize(Number(e.target.value));
                  }}
                >
                  {[10, 20, 30, 40, 50].map((pageSize) => (
                    <option
                      key={pageSize}
                      value={pageSize}
                      onClick={() =>
                        setPageLimit ? setPageLimit(pageSize) : {}
                      }
                    >
                      {pageSize}
                    </option>
                  ))}
                </select>
              </div> */}
              <div className="flex flex-row justify-center items-center gap-5">
                <span className="flex items-center justify-center gap-1">
                  <div className="text-right text-neutral-400 text-[12px] font-semibold uppercase tracking-wider">
                    {currentPage
                      ? currentPage
                      : table.getState().pagination.pageIndex + 1}{' '}
                    of {totalPage ? totalPage : table.getPageCount()}
                  </div>
                </span>
                <div className="flex gap-2">
                  <button
                    className="p-1 flex justify-center items-center"
                    onClick={() => {
                      if (currentPage && currentPage !== 1)
                        if (setCurrentPage) setCurrentPage(currentPage - 1);
                      table.previousPage();
                    }}
                    disabled={
                      currentPage === 1 ? !table.getCanPreviousPage() : false
                    }
                  >
                    <ChevronLeftIcon color="primary" width={16} height={16} />
                  </button>
                  <button
                    className="p-1 flex justify-center items-center"
                    onClick={() => {
                      if (currentPage && currentPage !== totalPage)
                        if (setCurrentPage) setCurrentPage(currentPage + 1);
                      table.nextPage();
                    }}
                    disabled={
                      currentPage && currentPage === totalPage
                        ? !table.getCanNextPage()
                        : false
                    }
                  >
                    <ChevronRightIcon color="primary" width={16} height={16} />
                  </button>
                </div>
              </div>
            </div>
          )}
        </div>
      )}
    </>
  );
};

export default Table;
