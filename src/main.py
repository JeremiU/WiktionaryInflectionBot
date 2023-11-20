import urllib.request

def find_line(arr, sub):
    j = -1

    for _x in range(len(arr)):
        if sub in arr[_x]:
            j = _x
            break

    return j

def url_bit(entry):
    k = (str(entry.encode('utf-8')).split('\'')[1]).replace("\\x", "%")
    urlStr = "https://en.wiktionary.org/wiki/" + k

    fp = urllib.request.urlopen(urlStr)
    mybytes = fp.read()

    mystr: str = mybytes.decode("utf8")
    fp.close()

    return mystr


def table(bit):
    y = list(bit.splitlines())
    z = find_line(y, '<h2><span class="mw-headline" id="Polish">Polish</span><span class="mw-editsection"><span class="mw-editsection-bracket">[')
    y = list(y)[z:]
    a = y.index('<table style="background:#F9F9F9; text-align:center; width: 100%; margin: 0;" class="inflection-table">')
    b = y.index('</td></tr></tbody></table></div></div>')
    y = list(y)[a:b]
    # z = list(filter(lambda x: "class=\"inflection-table\"" in x, y))
    return list(y)

def find_links(bit):
    nom_sg = 12
    nom_pl = 14
    gen_sg = 19
    gen_pl = 21
    dat_sg = 26
    dat_pl = 28
    acc_sg = 33
    acc_pl = 35
    ins_sg = 40
    ins_pl = 42
    loc_sg = 47
    loc_pl = 49
    voc_sg = 54
    voc_pl = 56

    # for k in range(len(bit)):
    #     print(f'{k}: {bit[k]}')
    #
    # print('\n\n\n')

    toCheck = []

    for k in [('nom sg', nom_sg), ('nom_pl', nom_pl), ('gen_sg', gen_sg), ('gen_pl', gen_pl), ('dat_sg', dat_sg), ('dat_pl', dat_pl), ('acc_sg', acc_sg), ('acc_pl', acc_pl),  ('ins_sg', ins_sg), ('ins_pl', ins_pl), ('loc_sg', loc_sg), ('loc_pl', loc_pl), ('voc_sg', voc_sg), ('voc_pl', voc_pl)]:
        strP = bit[k[1]]

        st = strP[strP.index('<a '):]

        if '(page does not exist)' in st:
            toCheck.append((k[0], st))

    return toCheck


def format(bit):
    ret = []
    for (ind, inf) in bit:
        i = inf.index('(page does not exist)">') + len('(page does not exist)">')
        j = inf.index('</a>')
        ret.append((ind, inf[i:j]))
    return ret

def gender_raw(bit):
    y = list(bit.splitlines())

    j = find_line(y, '<h2><span class="mw-headline" id="Polish">Polish</span><span class="mw-editsection"><span class="mw-editsection-bracket">')
    y = list(y)[j:]

    k = -1

    for _x in range(len(y)):
        if "<span class=\"gender\">" in y[_x]:
            k = _x
            break
    gen_line = y[k]

    l = gen_line.index('<span class="gender">') + len('<span class="gender">')
    k = gen_line.index('</span>')
    return gen_line[l:k]


def gender(bit):
    match bit:
        case '<abbr title=\"feminine gender\">f</abbr>':
            return 'f'
        case '<abbr title="neuter gender">n</abbr>':
            return 'n'
        case '<abbr title="masculine gender">m</abbr>&#160;<abbr title="animate">anim</abbr>':
            return 'm-a'
        case '<abbr title="masculine gender">m</abbr>&#160;<abbr title="inanimate">inan</abbr>':
            return 'm-i'
        case '<abbr title="masculine gender">m</abbr>&#160;<abbr title="personal">pers</abbr>':
            return 'm-p'

def process(wrd):
    url_data = url_bit(wrd)
    x = table(url_data)
    frm = format(find_links(x))

    newArr = []
#    for (k, v) in frm:
#        if (k, _) in newArr:
# need to filter duplicates


    print(f'gender: {gender(gender_raw(url_data))}')

    if len(frm) == 0:
        print(f"All forms of {wrd} exist!")
    else:
        print(f"existing pages: {13-len(frm)}/13")
        print(f"pages to create: {frm}")

    print('-'*10)


def gen_pg(bit):
    print("==Polish==")
    print("")
    print("===Pronunciation===")
    print("{{pl-p}}")
    print("")
    print("===Noun===")
    print("{{head|pl|noun|g=n}}")
    print("")
    print("# {{inflection of|pl|noun||case|p}}")

# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print('-'*10)
    process('wałówka')
    process('okno')
    process('koń')
    process('chłop')
    process('kwadrat')
