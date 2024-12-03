"use client"

import { componentSearch } from "@/constants/contents/search";
import { Button, Icon, Input, InputGroup, InputLeftElement, InputRightElement } from "@chakra-ui/react";
import { BiSearch } from "react-icons/bi";
import { FaX } from "react-icons/fa6";

interface SearchProps {
  defaultValue: string;
  onFillSearch: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onClearSearch: () => void;
}

export default function Search({
  defaultValue, onFillSearch, onClearSearch
}: SearchProps) {
  return (
    <InputGroup bg={'white'} rounded={'2xl'} size={'lg'}>
      <InputLeftElement pointerEvents='none'>
        <Icon as={BiSearch} color='#676767' mt={1} boxSize={5} />
      </InputLeftElement>
      <Input
        value={defaultValue}
        pr="3.3rem"
        onChange={onFillSearch}
        placeholder={componentSearch.placeholder}
        borderColor={'#A1A2A6'}
        rounded={'xl'}
        fontSize={'md'}
        _placeholder={{ color: "#6F6F6F" }}
      />
      {defaultValue.length > 0 && (
        <InputRightElement w={'3.5rem'}>
          <Button
            h='1rem'
            size='xs'
            rounded={'full'}
            py={3}
            onClick={onClearSearch}
          >
            <Icon as={FaX} boxSize={2} />
          </Button>
        </InputRightElement>
      )}
    </InputGroup>
  )
}