"use client"

import { Flex, Icon, Select, Text } from '@chakra-ui/react';
import { usePathname, useRouter, useSearchParams } from 'next/navigation';
import ReactPaginate from 'react-paginate';

import './_pagination.css';
import { FaChevronLeft, FaChevronRight } from 'react-icons/fa';

interface PaginationProps {
  currentPage: number;
  totalPages: number;
  limit: number;
}

export default function Pagination({
  currentPage,
  totalPages,
  limit
}: PaginationProps) {
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();

  const createPageURL = (pageNumber: number | string) => {
    const params = new URLSearchParams(searchParams);
    params.set('page', pageNumber.toString());

    const newUrl = `${pathname}?${params.toString()}`;
    router.push(newUrl);
  };

  const onDisplayData = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const { value } = e.target;
    const params = new URLSearchParams(searchParams);

    params.set('limit', value);
    params.set('page', '1');

    const newUrl = `${pathname}?${params.toString()}`;
    router.push(newUrl);
  };

  return (
    <Flex align={'center'} justify={'space-between'}>
      {totalPages > 1 && (
        <ReactPaginate
          breakLabel={"..."}

          activeClassName={'item active'}
          breakClassName={'item break-me'}
          containerClassName={`pagination`}
          pageClassName={'item pagination-page'}
          pageLinkClassName={`item link`}
          nextLinkClassName={`item link`}
          previousLinkClassName={`item link`}

          nextClassName={`item next`}
          nextLabel={<Icon as={FaChevronRight} boxSize={3} />}

          previousClassName={`item previous`}
          previousLabel={<Icon as={FaChevronLeft} boxSize={3} />}

          forcePage={currentPage - 1}
          pageCount={totalPages}
          pageRangeDisplayed={5}
          renderOnZeroPageCount={null}
          onPageChange={(page) => createPageURL(page.selected + 1)}
          onPageActive={(page) => createPageURL(page.selected + 1)}
        />
      )}

      <Flex w={'fit-content'} align={'center'} gap={5}>
        <Text
          fontSize={'md'}
          color={'neutral.700'}
          whiteSpace={'nowrap'}
          fontWeight={'medium'}
        >
          Show
        </Text>
        <Select defaultValue={limit} onChange={onDisplayData} bg={'neutral.100'}>
          <option value={10}>10</option>
          <option value={20}>20</option>
          <option value={50}>50</option>
          <option value={100}>100</option>
        </Select>
        <Text
          fontSize={'md'}
          color={'neutral.700'}
          whiteSpace={'nowrap'}
          fontWeight={'medium'}
        >
          Row per page
        </Text>
      </Flex>
    </Flex>
  )
}